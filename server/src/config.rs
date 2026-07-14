use std::env;

#[derive(Clone)]
pub struct Config {
    // Postgres
    pub database_url: String,

    // OAuth
    pub jwt_secret: String,
    pub access_token_ttl_seconds: i64,
    pub refresh_token_ttl_seconds: i64,
    pub verify_code_ttl_seconds: i64,

    // Google Cloud
    pub google_project_id: String,
    pub gemini_model: String,
    pub embedding_model: String,

    // Cloudflare
    pub cloudflare_account_id: String,
    pub cloudflare_bucket_access_key_id: String,
    pub cloudflare_bucket_name: String,
    pub cloudflare_bucket_secret_access_key: String,
    pub cloudflare_bucket_token_value: Option<String>,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            // Postgres
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),

            // OAuth
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
            access_token_ttl_seconds: 900,      // 15 min
            refresh_token_ttl_seconds: 604_800, // 7 days
            verify_code_ttl_seconds: 600,       // 1 hour // todo: 3600

            // Google Cloud
            google_project_id: env::var("GOOGLE_PROJECT_ID").expect("GOOGLE_PROJECT_ID must be set"),
            gemini_model: env::var("GEMINI_MODEL").expect("GEMINI_MODEL must be set"),
            embedding_model: env::var("EMBEDDING_MODEL").expect("EMBEDDING_MODEL must be set"),

            // Cloudflare R2
            cloudflare_account_id: env::var("CLOUDFLARE_ACCOUNT_ID").expect("CLOUDFLARE_ACCOUNT_ID must be set"),
            cloudflare_bucket_access_key_id: env::var("CLOUDFLARE_BUCKET_ACCESS_KEY_ID").expect("CLOUDFLARE_BUCKET_ACCESS_KEY_ID must be set"),
            cloudflare_bucket_name: env::var("CLOUDFLARE_BUCKET_NAME").expect("CLOUDFLARE_BUCKET_NAME must be set"),
            cloudflare_bucket_secret_access_key: env::var("CLOUDFLARE_BUCKET_SECRET_ACCESS_KEY").expect("CLOUDFLARE_BUCKET_SECRET_ACCESS_KEY must be set"),
            cloudflare_bucket_token_value: env::var("CLOUDFLARE_BUCKET_TOKEN_VALUE").ok().and_then(|val| {
                if val.trim().is_empty() { None } else { Some(val) }
            }),
        }
    }
}