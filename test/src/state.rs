use std::sync::Arc;

use s3::Bucket;
use sqlx::PgPool;

use crate::clients::email::Client as EmailClient;
use crate::clients::google_cloud::Client as GoogleCloudClient;
use crate::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub pool: PgPool,
    pub cloudflare_bucket: Arc<Bucket>,
    pub email_client: Arc<EmailClient>,
    pub google_cloud_client: Arc<GoogleCloudClient>,
}

impl AppState {
    pub async fn new(
        config: Config,
        pool: PgPool,
    ) -> Self {
        let config = Arc::new(config);

        let google_cloud_client = crate::clients::google_cloud::create_client()
            .await
            .expect("Failed to initialize Google Cloud Client");

        let cloudflare_bucket = crate::clients::cloudflare::create_cloudflare_bucket(&config)
            .expect("Failed to initialize Cloudflare R2 bucket");

        Self {
            config: config.clone(),
            pool,
            cloudflare_bucket: Arc::new(cloudflare_bucket),
            email_client: Arc::new(EmailClient::new()),
            google_cloud_client: Arc::new(google_cloud_client),
        }
    }
}