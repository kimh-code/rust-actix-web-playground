use crate::{
    error::Error,
};

pub struct JwtService;

impl JwtService {
    pub fn generate_token(user_id: &str) -> String {
        format!("fake_jwt_token_{}", user_id)
    }

    pub fn verify_token(token: &str) -> Result<String, Error> {
        if token.starts_with("fake_jwt_token_") {
            let user_id = token.replace("fake_jwt_token_", "");
            Ok(user_id)
        } else {
            Err(Error::Unauthorized("Invalid token".to_string()))
        }
    }
}