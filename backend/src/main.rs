use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema, Request, Response};
use axum::{
    extract::State,
    response::{self, IntoResponse},
    routing::get,
    Router, Server, Json
};
use sqlx::postgres::PgPoolOptions;
use dotenvy::dotenv;

mod model;
use crate::model::mutation::MutationRoot;
use crate::model::query::QueryRoot;

type GakushockerSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

async fn graphql_handler(
    schema: State<GakushockerSchema>,
    req: Json<Request>,
) -> Json<Response> {
    schema.execute(req.0).await.into()
}

async fn graphql() -> impl IntoResponse {
    response::Html(
        GraphiQLSource::build()
            .endpoint("http://localhost:8000")
            .finish(),
    )
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").unwrap();
    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .unwrap();

    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(db)
        .finish();

    let app = Router::new()
        .route("/", get(graphql).post(graphql_handler))
        .with_state(schema);

    println!("🚀 GraphQL IDE: http://localhost:8000");

    Server::bind(&"127.0.0.1:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
