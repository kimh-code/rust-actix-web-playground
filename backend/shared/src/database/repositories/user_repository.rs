use crate::{
    models::user::User,
    database::models::db_user::DbUser,
};
use sqlx::PgPool;
use uuid::Uuid;
use async_graphql::ID;

pub struct UserRepository{
    pool: PgPool,
}

impl UserRepository {

    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_id(&self, id: &ID) -> Result<Option<DbUser>, sqlx::Error> {

        let uuid_id = Uuid::parse_str(&id.0)
            .map_err(|_| sqlx::Error::RowNotFound)?;

        sqlx::query_as::<_, DbUser>(
            "SELECT * FROM users WHERE id = $1 AND is_deleted = false"
        )
        .bind(uuid_id)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn find_by_ids(&self, ids: &[ID]) -> Result<Vec<DbUser>, sqlx::Error> {
        let numeric_ids: Vec<Uuid> = ids.iter()
            .filter_map(|id| id.0.parse().ok())
            .collect();

        if numeric_ids.is_empty() {
            return Ok(vec![]);
        }

        let users = sqlx::query_as::<_, DbUser>(
            "SELECT * FROM users WHERE id = ANY($1) AND is_deleted = false"
        )
        .bind(&numeric_ids)
        .fetch_all(&self.pool)
        .await?;

        Ok(users)
    }

    pub async fn create(
        &self,
        username: &str,
        email: &str,
        password_hash: &str
    ) -> Result<DbUser, sqlx::Error> {
        sqlx::query_as::<_, DbUser>(
            "INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3) RETURNING id, username, email, password_hash, is_deleted, internal_notes, created_at, updated_at"
        )
        .bind(username)
        .bind(email)
        .bind(password_hash) // 실제 hash기능 나중에 추가하기
        .fetch_one(&self.pool)
        .await
    }

    pub async fn find_all(&self) -> Result<Vec<DbUser>, sqlx::Error> {
        sqlx::query_as::<_, DbUser>("SELECT * FROM users")
            .fetch_all(&self.pool)
            .await
    }
}