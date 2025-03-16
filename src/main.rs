use askama::Template;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Router;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/", get(index))
        .route("/hello", get(hello))
        .nest_service("/assets", ServeDir::new("assets"));

    let listner = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listner, router).await.unwrap();
}

async fn index() -> impl IntoResponse {
    let template = IndexTemplate { name: "Marcin" };

    Html(template.render().unwrap())
}

async fn hello() -> impl IntoResponse {
    let template = HelloTemplate {};

    Html(template.render().unwrap())
}

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    name: &'a str,
}
