use sqlx::{PgPool, query};
use uuid::Uuid;

use crate::error::Result;
use crate::history::model::HistoryType;

pub async fn create(
    pool: &PgPool,
    user_id: Uuid,
    target_id: Uuid,
    r#type: HistoryType,
) -> Result<()> {
    query(
        r#"
        INSERT INTO history (user_id, target_id, type)
        VALUES ($1, $2, $3)
        "#,
    )
    .bind(user_id)
    .bind(target_id)
    .bind(r#type)
    .execute(pool)
    .await?;
    Ok(())
}
