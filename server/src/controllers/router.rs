use crate::database;
use crate::database::RepositoryProvider;
use crate::presenters::mutation::Mutation;
use crate::presenters::query::Query;
use async_graphql::{http::GraphiQLSource, EmptySubscription, Request, Response, Schema};
use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
    Json, Router,
};
use http::Method;
use tower_http::cors::{Any, CorsLayer};

type GakushockerSchema = Schema<Query, Mutation, EmptySubscription>;

pub async fn root() -> Router {
    let cors = cors();

    let pool = database::db_pool().await;
    let repository_provider = RepositoryProvider(pool);
    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(repository_provider)
        .finish();

    Router::new()
        .route("/", get(graphql).post(graphql_handler))
        .with_state(schema)
        .layer(cors)
}

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

fn cors() -> CorsLayer {
    CorsLayer::new()
        .allow_methods(vec![Method::GET, Method::POST, Method::OPTIONS])
        .allow_origin(vec![
            "http://localhost".parse::<http::HeaderValue>().unwrap(),
            "http://localhost:3000"
                .parse::<http::HeaderValue>()
                .unwrap(),
            "http://localhost:8080"
                .parse::<http::HeaderValue>()
                .unwrap(),
        ])
        .allow_headers(Any)
}
