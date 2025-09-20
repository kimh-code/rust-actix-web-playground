use crate::{
    error::Error,
    models::user::UserProfile,
    database::repositories::user_repository::UserRepository,
};

#[derive(Clone)]
pub struct UserService {
    user_repo: UserRepository,
}

impl UserService {
    pub fn new(user_repo: UserRepository) -> Self {
        Self { user_repo }
    }

    pub async fn create(
        &self,
        username: &str,
        email: &str,
        password_hash: &str
    ) -> Result<UserProfile, Error> {
        let db_user = self.user_repo.create(&username, &email, &password_hash).await?;

        Ok(UserProfile::from(db_user))
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<UserProfile>, sqlx::Error> {
        let db_user = self.user_repo.find_by_id(id).await?;
        let user_profile = db_user.map(UserProfile::from);

        Ok(user_profile)
    }

    pub async fn find_by_ids(&self, ids: &[&str]) -> Result<Vec<UserProfile>, sqlx::Error> {

        let db_users = self.user_repo.find_by_ids(ids).await?;
        let user_profiles: Vec<UserProfile> = db_users
            .into_iter()
            .map(UserProfile::from)
            .collect();

        Ok(user_profiles)
    }

    pub async fn get_user_profile(&self, user_id: &str) -> Result<UserProfile, Error> {
        if let Some(db_user) = self.user_repo.find_by_id(user_id).await? {
            Ok(UserProfile::from(db_user))
        } else {
            Err(Error::NotFound("해당 id의 사용자 없음".to_string()))
        }
    }

    pub async fn find_all(&self) -> Result<Vec<UserProfile>, sqlx::Error> {
        let db_users = self.user_repo.find_all().await?;
        
        let user_profiles: Vec<UserProfile> = db_users
            .into_iter()
            .map(UserProfile::from)
            .collect();

        Ok(user_profiles)
    }
}