use actix_web::{web, App, HttpServer, middleware::from_fn,
};
use shared::{
    database::{
        apply_migration::MigrationManager,
        repositories::user_repository::UserRepository,
        services::user_service::UserService,
    },
    error::Error as AppError,
    auth::{
        middleware::auth_middleware, jwt_service::JwtService, auth_service::AuthService,
    },
};
use sqlx::PgPool;
use std::env;
use dotenv::dotenv;
mod handlers;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    println!("서버 시작 중...");

    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL 환경변수가 설정되지 않았습니다");

    println!("데이터베이스 연결 중...");
    let pool = PgPool::connect(&database_url).await.unwrap();

    println!("마이그레이션 실행 중...");
    println!("CARGO_MANIFEST_DIR: '{}'", env!("CARGO_MANIFEST_DIR"));
    let migrations_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../shared/src/database/migrations");
    println!("계산된 마이그레이션 경로: '{}'", migrations_dir);
    let migration_manager = MigrationManager::new(migrations_dir.to_string()).await?;

    MigrationManager::ensure_migration_table(&pool).await?;
    migration_manager.run_pending_up_migrations(&pool).await?;

    println!("마이그레이션 완료!");

    let user_repo = UserRepository::new(pool.clone());
    let user_service = UserService::new(user_repo.clone());
    let auth_service = AuthService::new(user_repo.clone());

    let test_token = JwtService::generate_token("25770f6b-869e-4870-87c8-ecd5c24395e0");
    println!("Test token: {}", test_token);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(user_service.clone()))
            .app_data(web::Data::new(auth_service.clone()))
            .wrap(from_fn(auth_middleware))

            .service(
                web::scope("/api/v1")
                    .route("me", web::get().to(handlers::get_me))
                    .route("/users/{id}", web::get().to(handlers::get_user))
                    .route("/users", web::post().to(handlers::create_user))
                    .route("/health", web::get().to(handlers::health_check))
            )
    })
    .bind("127.0.0.1:8001")?
    .run()
    .await?;

    Ok(())
}