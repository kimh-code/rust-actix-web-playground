use crate::{
    auth::{JwtService, AuthService},
};
use actix_web::{
    middleware::Next,
    dev::{ServiceRequest, ServiceResponse},
    Error as ActixError,
    HttpMessage,
    body::MessageBody,
    web,
};

pub async fn auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, ActixError> {

    println!("[DEBUG] Auth middleware called for: {}", req.path());
    if let Some(auth_header) = req.headers().get("Authorization") {
        println!("[DEBUG] Authorization header: {:?}", auth_header);
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                println!("추출된 토큰: {}", token);

                match JwtService::verify_token(token) {
                    Ok(user_id) => {
                        println!("추출된 user_id: {}", user_id);

                        if let Some(auth_service) = req.app_data::<web::Data<AuthService>>() {
                            match auth_service.create_current_user_by_id(&user_id).await {
                                Ok(current_user) => {
                                    req.extensions_mut().insert(current_user);
                                }
                                Err(_) => {
                                    return Err(actix_web::error::ErrorUnauthorized("User not found"));
                                }
                            }
                        } else {
                            return Err(actix_web::error::ErrorInternalServerError("UserRepository not found"));
                        }
                    }
                    Err(_) => {
                        return Err(actix_web::error::ErrorUnauthorized("Invalid token"))
                    }
                }
            }
        }
    }

    next.call(req).await
}