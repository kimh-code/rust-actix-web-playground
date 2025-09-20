use actix_web::{web, HttpRequest, HttpResponse, Result, HttpMessage};
use shared::{
    auth::current_user::CurrentUser,
    models::{user::RestUser,
            request::CreateUserRequest,
    },
    database::services::user_service::UserService,
};
use serde_json::json;

pub async fn create_user(
    user_data: web::Json<CreateUserRequest>,
    user_service: web::Data<UserService>
) -> Result<HttpResponse> {
    let request = user_data.into_inner();

    let password_hash = format!("hashed_{}", request.password); // 임시

    let user_profile = user_service.create(&request.username, &request.email, &password_hash).await?;

    let rest_user = RestUser::from(user_profile);
    Ok(HttpResponse::Created().json(rest_user))
}

pub async fn get_me(
    req: HttpRequest,
    user_service: web::Data<UserService>
) -> Result<HttpResponse> {
    let current_user = req.extensions().get::<CurrentUser>()
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?
        .clone();

    let user_profile = user_service.find_by_id(&current_user.id.to_string()).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?
        .ok_or_else(|| actix_web::error::ErrorNotFound("User not found"))?;

    let rest_user = RestUser::from(user_profile);

    Ok(HttpResponse::Ok().json(rest_user))
}

pub async fn get_user(
    path: web::Path<String>,
    user_service: web::Data<UserService>
) -> Result<HttpResponse> {
    let user_id = path.into_inner();

    match user_service.find_by_id(&user_id).await {
        Ok(Some(profile)) => {
            let rest_user = RestUser::from(profile);
            Ok(HttpResponse::Ok().json(rest_user))
        },
        Ok(None) => Ok(HttpResponse::NotFound().json(json!({
            "error": "User not found"
        }))),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}

pub async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({
        "status": "ok",
        "service": "REST API",
        "timestamp": chrono::Utc::now()
    })))
}