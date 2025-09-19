use crate::{
    models::user::GraphQLUser,
    database::models::db_user::DbUser,
};
use sqlx::PgPool;
use uuid::Uuid;
use async_graphql::ID;

#[derive(Clone)]
pub struct UserRepository{
    pool: PgPool,
}

impl UserRepository {

    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<DbUser>, sqlx::Error> {

        let uuid_id = Uuid::parse_str(id)
            .map_err(|_| sqlx::Error::RowNotFound)?;

        sqlx::query_as::<_, DbUser>(
            "SELECT * FROM users WHERE id = $1 AND is_deleted = false"
        )
        .bind(uuid_id)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn find_by_ids(&self, ids: &[&str]) -> Result<Vec<DbUser>, sqlx::Error> {
        let numeric_ids: Vec<Uuid> = ids.iter()
            .filter_map(|id| id.parse().ok())
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

    pub async fn find_roles_by_id(&self, user_id: &str) -> Result<Vec<String>, sqlx::Error> {
        let uuid = Uuid::parse_str(user_id)
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;

        let role_names = sqlx::query!(
            "SELECT r.name
             FROM user_roles ur
             JOIN roles r ON ur.role_id = r.id
             WHERE ur.user_id = $1",
             uuid
            )
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|row| row.name)
            .collect::<Vec<String>>();

        Ok(role_names)
    }

    pub async fn find_user_with_roles(&self, user_id: &str) -> Result<Option<(DbUser, Vec<String>)>, sqlx::Error> {
        let user = self.find_by_id(user_id).await?;

        if let Some(user) = user {
            let role_names = self.find_roles_by_id(user_id).await?;

            Ok(Some((user, role_names)))
        } else {
            Ok(None)
        }
    }
}