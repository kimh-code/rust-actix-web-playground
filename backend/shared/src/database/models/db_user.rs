use time::OffsetDateTime;
use uuid::Uuid;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct DbUser {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub is_deleted: bool,
    pub internal_notes: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}