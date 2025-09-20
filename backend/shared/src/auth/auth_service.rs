use crate::{
    error::Error,
    auth::{
        CurrentUser,
    },
    database::repositories::user_repository::UserRepository,
};

#[derive(Clone)]
pub struct AuthService {
    user_repo: UserRepository,
}

impl AuthService {
    pub fn new(user_repo: UserRepository) -> Self {
        Self { user_repo }
    }

    pub async fn create_current_user_by_id(&self, user_id: &str) -> Result<CurrentUser, Error> {
        let (db_user, role_names) = self.user_repo.find_user_with_roles(&user_id).await
            .map_err(|e| Error::Database(e))?
            .ok_or(Error::NotFound("User not found".to_string()))?;

        let current_user = CurrentUser::from((db_user,role_names));

        Ok(current_user)
    }
}