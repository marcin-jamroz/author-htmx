use axum::routing::get;
use axum::Router;
use tower_http::services::ServeDir;

mod routes;
use routes::{hello::hello::hello, home::home::home, index::index::index};

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/", get(index))
        .route("/home", get(home))
        .route("/hello", get(hello))
        .nest_service("/assets", ServeDir::new("assets"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
