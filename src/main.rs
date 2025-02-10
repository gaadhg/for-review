#![allow(unused)]
mod domain;
mod infrastructure;
mod presentation;

use std::sync::Arc;
use time::Duration;

use axum_login::{
    AuthManagerLayerBuilder, login_required,
    tower_sessions::{Expiry, MemoryStore, SessionManagerLayer, cookie::SameSite},
};

use axum::http::{StatusCode, Uri, header};
use axum::{
    extract::FromRef,
    http::{HeaderMap, HeaderValue},
    response::IntoResponse,
    routing::*,
    *,
};
use grass::include;
use infrastructure::*;
use presentation::routes::*;
use tera::Tera;
use tower_http::services::ServeDir;
// #[derive(Clone, FromRef)]
// struct AppState {
//     views: Arc<Tera>,
// }

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_same_site(SameSite::Lax)
        .with_expiry(Expiry::OnInactivity(Duration::days(1)));

    let repository = StudentRepository::new().unwrap();
    let auth_layer = AuthManagerLayerBuilder::new(repository, session_layer).build();

    let app = Router::new()
        .route("/style.css", get(css))
        .nest_service("/js", ServeDir::new("src\\presentation\\templates\\js"))
        .route("/", get(home))
        .route("/app", get(app))
        // user handlers
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", get(logout))

        // partials
        .route("/app/dashboard", get(dashboard))
        .route("/app/cards", get(cards))
        .route("/app/schedule", get(schedule))
        .route("/app/calendar", get(calendar))
        .layer(auth_layer);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

// don't know where to place this
async fn css() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_str("text/css").unwrap(),
    );

    (
        headers,
        include!("src/presentation/templates/style/style.scss"),
    )
}
