use axum::Server;
use server::controllers::router;

#[tokio::main]
async fn main() {
    let app = router::root().await;

    println!("🚀 GraphQL IDE: http://localhost:8080");

    Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
