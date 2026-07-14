use std::sync::Arc;

use reqwest::Client as HttpClient;
use serde_json::json;
use yup_oauth2::{authenticator::DefaultAuthenticator, ServiceAccountAuthenticator};

use crate::db::text::{get_text, set_text};
use crate::error::{Error, Result};
use crate::models::text::Text;
use crate::state::AppState;

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

pub async fn get_embedding(
    state: &AppState,
    text: String,
) -> Result<Text> {
    let cache = get_text(&state.pool, &text).await?;
    if let Some(cache) = cache {
        return Ok(cache);
    }

    let url = format!(
        "https://us-central1-aiplatform.googleapis.com/v1/projects/{}/locations/us-central1/publishers/google/models/{}:predict",
        state.config.google_project_id,
        state.config.embedding_model,
    );

    let scopes = &["https://www.googleapis.com/auth/cloud-platform"];
    let token = state.google_cloud_client.auth.token(scopes).await
        .map_err(|e| Error::Internal(format!("Failed to get Google Cloud auth token: {}", e)))?;

    let request_body = json!({
        "instances": [
            { "text": text }
        ],
        "parameters": {
            "dimension": 512
        }
    });

    let response = state.google_cloud_client.http
        .post(&url)
        .bearer_auth(token.token().ok_or_else(|| Error::Internal("Token is empty".into()))?)
        .json(&request_body)
        .send()
        .await
        .map_err(|e| Error::Internal(format!("Failed to call Vertex AI embedding API: {}", e)))?;

    if !response.status().is_success() {
        let error_body = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        return Err(Error::Internal(format!("Vertex AI API returned an error: {}", error_body)));
    }

    let body: serde_json::Value = response.json().await
        .map_err(|e| Error::Internal(format!("Failed to parse Vertex AI response: {}", e)))?;

    let embedding_vec = body["predictions"]
        .get(0)
        .and_then(|pred| pred["textEmbedding"].as_array())
        .ok_or_else(|| Error::Internal("Could not find 'textEmbedding' in Vertex AI response".into()))?;

    let embedding: Vec<f32> = embedding_vec
        .iter()
        .filter_map(|v| v.as_f64().map(|f| f as f32))
        .collect();

    if embedding.len() != 512 {
        return Err(Error::Internal(format!("Expected embedding of dimension 512, but got {}", embedding.len())));
    }

    let embedding = embedding.into();
    
    let cache = set_text(&state.pool, text, embedding).await?;
    Ok(cache)
}