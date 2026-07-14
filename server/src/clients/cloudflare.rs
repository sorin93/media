use s3::{creds::Credentials, Bucket, Region};

use crate::config::Config;
use crate::error::{Error, Result};

pub fn create_cloudflare_bucket(config: &Config) -> Result<Bucket> {
    let region = Region::R2 {
        account_id: config.cloudflare_account_id.clone(),
    };

    let credentials = Credentials::new(
        Some(&config.cloudflare_bucket_access_key_id),
        Some(&config.cloudflare_bucket_secret_access_key),
        None, // R2 does not use session tokens (X-Amz-Security-Token), passing one causes InvalidArgument errors
        None,
        None,
    ).map_err(|e| Error::Internal(format!("Failed to create Cloudflare R2 credentials: {}", e)))?;

    let bucket = Bucket::new(&config.cloudflare_bucket_name, region, credentials)
        .map_err(|e| Error::Internal(format!("Failed to create R2 bucket instance: {}", e)))?;

    tracing::info!("Cloudflare R2 bucket initialized successfully");

    Ok(*bucket)
}

pub async fn delete_cloudflare_bucket_folder(_bucket: &Bucket, _folder: &str) -> Result<()> {
    // todo
    /*
    bucket
        .put_object_with_content_type(path, content, content_type)
        .await
        .map_err(|e| Error::Internal(format!("Failed to upload to R2: {}", e)))?;
    */
    Ok(())
}