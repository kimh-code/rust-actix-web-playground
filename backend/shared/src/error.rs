use thiserror::Error;
use actix_web::HttpResponse;
use serde_json::json;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Server error: {0}")]
    Server(String),

    #[error("Invalid Input error: {0}")]
    InvalidInput(String),
}

impl actix_web::ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::Unauthorized(msg) => HttpResponse::Unauthorized().json(json!({
                "error": msg
            })),
            Error::Forbidden(msg) => HttpResponse::Unauthorized().json(json!({
                "error": msg
            })),
            Error::NotFound(msg) => HttpResponse::NotFound().json(json!({
                "error": msg
            })),
            Error::Database(_) => HttpResponse::InternalServerError().json(json!({
                "error": "Internal server error"
            })),
            Error::Validation(msg) => HttpResponse::BadRequest().json(json!({
                "error": msg
            })),
            Error::Io(_) => HttpResponse::InternalServerError().json(json!({
                "error": "Internal server error"
            })),
            Error::Server(msg) => HttpResponse::NotFound().json(json!({
                "error": msg
            })),
            Error::InvalidInput(msg) => HttpResponse::BadRequest().json(json!({
                "error": msg
            })),
        }
    }
}