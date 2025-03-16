use askama::Template;
use axum::response::{Html, IntoResponse};

pub async fn home() -> impl IntoResponse {
    let template = HomeTemplate { name: "Marcin" };

    Html(template.render().unwrap())
}

#[derive(Template)]
#[template(path = "routes/home/home.html")]
struct HomeTemplate<'a> {
    name: &'a str,
}
