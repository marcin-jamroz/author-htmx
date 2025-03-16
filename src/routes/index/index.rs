use askama::Template;
use axum::response::{Html, IntoResponse};

pub async fn index() -> impl IntoResponse {
    let template = IndexTemplate {};
    Html(template.render().unwrap())
}

#[derive(Template)]
#[template(path = "routes/index/index.html")]
struct IndexTemplate;
