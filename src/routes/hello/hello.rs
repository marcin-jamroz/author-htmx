use askama::Template;
use axum::response::{Html, IntoResponse};

pub async fn hello() -> impl IntoResponse {
    let template = HelloTemplate {};

    Html(template.render().unwrap())
}

#[derive(Template)]
#[template(path = "routes/hello/hello.html")]
struct HelloTemplate;
