use sqlx::types::chrono::DateTime;
use sqlx::FromRow;
use sqlx::types::chrono::Utc;

#[derive(Debug, FromRow)]
pub struct SchemaMigration {
    pub version: i64,
    pub applied_at: DateTime<Utc>,
}