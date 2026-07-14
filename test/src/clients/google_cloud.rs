use std::sync::Arc;

use reqwest::Client as HttpClient;
use serde_json::json;
use yup_oauth2::{ServiceAccountAuthenticator, authenticator::DefaultAuthenticator};

use crate::error::{Error, Result};
use crate::state::AppState;
use crate::text::model::Text;
use crate::text::repository;

pub type Auth = DefaultAuthenticator;

#[derive(Clone)]
pub struct Client {
    pub auth: Arc<Auth>,
    pub http: Arc<HttpClient>,
}

pub async fn create_client() -> Result<Client> {
    let sa_key_file = std::env::var("GOOGLE_APPLICATION_CREDENTIALS")
        .unwrap_or_else(|_| "google-cloud.json".to_string());

    let sa_key = yup_oauth2::read_service_account_key(&sa_key_file)
        .await
        .map_err(|e| Error::Internal(format!("Failed to read service account key: {e}")))?;

    let auth = ServiceAccountAuthenticator::builder(sa_key)
        .build()
        .await
        .map_err(|e| Error::Internal(format!("Failed to create authenticator: {e}")))?;

    tracing::info!("Google Cloud client initialized successfully");

    Ok(Client {
        auth: Arc::new(auth),
        http: Arc::new(HttpClient::new()),
    })
}

pub async fn get_embedding(state: &AppState, text: String) -> Result<Text> {
    // 1. Check Cache
    let cache = repository::find_by_text(&state.pool, &text).await?;
    if let Some(cache) = cache {
        return Ok(cache);
    }

    // 2. Updated URL to use :embedContent
    let url = format!(
        "https://us-central1-aiplatform.googleapis.com/v1/projects/{}/locations/us-central1/publishers/google/models/{}:embedContent",
        state.config.google_project_id,
        state.config.embedding_model, // (Ensure your .env has gemini-embedding-2-preview)
    );

    let scopes = &["https://www.googleapis.com/auth/cloud-platform"];
    let token = state
        .google_cloud_client
        .auth
        .token(scopes)
        .await
        .map_err(|e| Error::Internal(format!("Failed to get Google Cloud auth token: {}", e)))?;

    // 3. Updated JSON payload for Gemini Text Embeddings
    // Note: We use `&text` so the macro borrows it instead of consuming the String,
    // because we still need to pass `text` to `set_text` at the bottom.
    let request_body = json!({
        "content": {
            "parts": [
                { "text": &text }
            ]
        },
        "outputDimensionality": 3072
    });

    let response = state
        .google_cloud_client
        .http
        .post(&url)
        .bearer_auth(
            token
                .token()
                .ok_or_else(|| Error::Internal("Token is empty".into()))?,
        )
        .json(&request_body)
        .send()
        .await
        .map_err(|e| Error::Internal(format!("Failed to call Vertex AI embedding API: {}", e)))?;

    if !response.status().is_success() {
        let error_body = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(Error::Internal(format!(
            "Vertex AI API returned an error: {}",
            error_body
        )));
    }

    let body: serde_json::Value = response
        .json()
        .await
        .map_err(|e| Error::Internal(format!("Failed to parse Vertex AI response: {}", e)))?;

    // 4. Extract the embedding vector from the Gemini response structure
    let embedding_vec = body["embedding"]["values"].as_array().ok_or_else(|| {
        Error::Internal("Could not find 'embedding.values' in Vertex AI response".into())
    })?;

    let embedding: Vec<f32> = embedding_vec
        .iter()
        .filter_map(|v| v.as_f64().map(|f| f as f32))
        .collect();

    // 5. Update validation to expect the full 3072 dimensions
    if embedding.len() != 3072 {
        return Err(Error::Internal(format!(
            "Expected embedding of dimension 3072, but got {}",
            embedding.len()
        )));
    }

    // Convert and save to database
    let cache = repository::insert(&state.pool, text, embedding.into()).await?;

    Ok(cache)
}
