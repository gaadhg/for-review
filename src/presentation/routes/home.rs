use super::TEMPLATES;
use axum::response::{Html, IntoResponse};
use tera::{Context, Tera};

pub async fn home() -> impl IntoResponse {
    Html(TEMPLATES.render("page/home.html", &Context::new()).unwrap())
}
