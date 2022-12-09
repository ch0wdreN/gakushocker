use async_graphql::{http::GraphiQLSource, EmptySubscription, Request, Response, Schema};
use axum::{
    extract::State,
    response::{IntoResponse, Html},
    routing::get,
    Json, Router, Server,
};
use dotenvy::dotenv;
use http::Method;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::{Any, CorsLayer};

mod model;
use crate::model::mutation::Mutation;
use crate::model::query::Query;

type GakushockerSchema = Schema<Query, Mutation, EmptySubscription>;

async fn graphql_handler(schema: State<GakushockerSchema>, req: Json<Request>) -> Json<Response> {
    schema.execute(req.0).await.into()
}

async fn graphql() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint("http://localhost:8080")
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

    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(db)
        .finish();

    let cors = CorsLayer::new()
        .allow_methods(vec![Method::GET, Method::POST, Method::OPTIONS])
        .allow_origin(vec![
            "http://localhost:3000"
                .parse::<http::HeaderValue>()
                .unwrap(),
            "http://127.0.0.1:3000"
                .parse::<http::HeaderValue>()
                .unwrap(),
            "http://localhost:8080"
                .parse::<http::HeaderValue>()
                .unwrap(),
            "http://localhost"
                .parse::<http::HeaderValue>()
                .unwrap(),
        ])
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(graphql).post(graphql_handler))
        .with_state(schema)
        .layer(cors);

    println!("🚀 GraphQL IDE: http://localhost:8080");

    Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
