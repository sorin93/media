use sqlx::PgPool;

use crate::error::Error;
use crate::error::Result;

pub async fn run_migrations(pool: &PgPool) -> Result<()> {
    match sqlx::migrate!("./migrations").run(pool).await {
        Ok(_) => {
            tracing::info!("Database migrations completed successfully");
            Ok(())
        }
        Err(e) => {
            tracing::error!("Migration error: {:?}", e);
            Err(Error::Internal("Database migration failed".into()))
        }
    }
}
