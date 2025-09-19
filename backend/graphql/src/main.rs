use async_graphql::*;
use async_graphql_actix_web::{GraphQL, GraphQLSubscription, GraphQLRequest, GraphQLResponse};
use actix_web::{web, App, HttpServer, HttpResponse, Result as ActixResult,
                middleware::from_fn, HttpRequest, HttpMessage,
};
use shared::{
    database::{
        apply_migration::MigrationManager,
        repositories::user_repository::UserRepository,
    },
    models::{
        user::{
            GraphQLUser, TimeOffsetDateTime,
        },
        mutation::Mutation,
    },
    error::Error as AppError,
    auth::{middleware::auth_middleware, CurrentUser, jwt_service::JwtService},
};
use sqlx::{Row,
            types::chrono,
            PgPool, Pool, Postgres, Transaction, Executor,
};
use std::env;
use dotenv::dotenv;

pub type MySchema = Schema<QueryRoot, Mutation, EmptySubscription>;

pub async fn graphql_handler(
    schema: web::Data<MySchema>,
    req: HttpRequest,
    payload: GraphQLRequest,
) -> GraphQLResponse {
    
    let mut graphql_request = payload.into_inner();

    if let Some(current_user) = req.extensions().get::<CurrentUser>() {
        graphql_request = graphql_request.data(current_user.clone());
    }

    let response = schema.execute(graphql_request).await;
    
    response.into()
}

struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hello(&self) -> &str {
        "Hello, GraphQL!"
    }

    async fn me(&self, ctx: &Context<'_>) -> Result<GraphQLUser> {
        let current_user = ctx.data::<CurrentUser>()
            .map_err(|_| "Not authenticated")?;

        Ok(GraphQLUser {
            id: current_user.id.into(),
            username: current_user.username.clone(),
            email: current_user.email.clone(),
            created_at: TimeOffsetDateTime(current_user.created_at),
            updated_at: TimeOffsetDateTime(current_user.updated_at),
        })
    }

    async fn user(&self, ctx: &Context<'_>, id: ID) -> Result<Option<GraphQLUser>> {
        let user_repo = ctx.data::<UserRepository>()?;

        if let Some(db_user) = user_repo.find_by_id(&id).await? {
            Ok(Some(db_user.into()))
        } else {
            Ok(None)
        }
    }

    async fn users(&self, ctx: &Context<'_>, ids: Vec<ID>) -> Result<Vec<GraphQLUser>> {
        let user_repo = ctx.data::<UserRepository>()?;

        let db_users = user_repo.find_by_ids(&ids).await?;
        let users: Vec<GraphQLUser> = db_users.into_iter().map(|db_user|db_user.into()).collect();

        Ok(users)
    }

    async fn find_all(&self, ctx: &Context<'_>) -> Result<Vec<GraphQLUser>> {
        let user_repo = ctx.data::<UserRepository>()?;

        let db_users = user_repo.find_all().await?;
        let users: Vec<GraphQLUser> = db_users.into_iter().map(|db_user|db_user.into()).collect();

        Ok(users)
    }
}

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

    let schema: MySchema = Schema::build(QueryRoot, Mutation::default(), EmptySubscription)
        .data(pool.clone())
        .data(user_repo.clone())
        .finish();

    let test_token = JwtService::generate_token("25770f6b-869e-4870-87c8-ecd5c24395e0");
    println!("Test token: {}", test_token);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(user_repo.clone()))
            .app_data(web::Data::new(schema.clone()))
            .wrap(from_fn(auth_middleware))
            .route("/", web::get().to(|| async {
                HttpResponse::Found()
                    .append_header(("Location", "/playground"))
                    .finish()
            }))
            .route("/graphql", web::post().to(graphql_handler))
            .route("/playground", web::get().to(index_graphiql))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await?;

    Ok(())
}

async fn index_graphiql() -> ActixResult<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            async_graphql::http::GraphiQLSource::build()
                .endpoint("/graphql")
                .finish()
        ))
}