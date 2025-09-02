use async_graphql::*;
use async_graphql_actix_web::{GraphQL, GraphQLSubscription};
use actix_web::{web, App, HttpServer, Result, HttpResponse};

struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hello(&self) -> &str {
        "Hello, GraphQL!"
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .finish();
    
    HttpServer::new(move || {
        App::new()
            .route("/graphql", web::post().to(GraphQL::new(schema.clone())))
            .route("/playground", web::get().to(index_graphiql))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

async fn index_graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            async_graphql::http::GraphiQLSource::build()
                .endpoint("/graphql")
                .finish()
        ))
}
