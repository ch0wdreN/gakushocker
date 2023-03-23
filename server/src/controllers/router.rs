use crate::controllers::auth::auth::{auth_router, validation_token};
use crate::controllers::presenters::mutation::Mutation;
use crate::controllers::presenters::query::Query;
use crate::database;
use crate::database::RepositoryProvider;
use async_graphql::{http::GraphiQLSource, EmptySubscription, Request, Response, Schema};
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Json, Router,
};
use axum_extra::extract::CookieJar;
use tower_http::cors::CorsLayer;

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
        .nest("/auth", auth_router())
        .layer(cors)
}

async fn graphql_handler(
    schema: State<GakushockerSchema>,
    jar: CookieJar,
    req: Json<Request>,
) -> Result<Json<Response>, StatusCode> {
    if let Some(token) = jar.get("token").map(|cookie| cookie.value().to_owned()) {
        if validation_token(token) {
            Ok(schema.execute(req.0).await.into())
        } else {
            return Err(StatusCode::UNAUTHORIZED);
        }
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    }
}

async fn graphql() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint("http://localhost:8080")
            .finish(),
    )
}

fn cors() -> CorsLayer {
    // CorsLayer::new()
    //     .allow_methods(vec![Method::GET, Method::POST, Method::OPTIONS])
    //     .allow_origin(vec![
    //         "http://localhost".parse::<http::HeaderValue>().unwrap(),
    //         "http://localhost:3000"
    //             .parse::<http::HeaderValue>()
    //             .unwrap(),
    //         "http://localhost:8080"
    //             .parse::<http::HeaderValue>()
    //             .unwrap(),
    //     ])
    //     .allow_headers(Any)
    CorsLayer::very_permissive()
}
