use super::TEMPLATES;
use crate::infrastructure::AuthSession;
use axum::response::{Html, IntoResponse};
use tera::Context;

pub async fn app(auth_session: AuthSession) -> impl IntoResponse {
    Html(TEMPLATES
    .render("page/app.html", &Context::new())
    .unwrap())
    // match auth_session.user {
    //     Some(user) => Html(
    //         TEMPLATES
    //             .render("page/app.html", &Context::new())
    //             .unwrap(),
    //     ),
    //     None => Html(
    //         TEMPLATES
    //             .render("page/login.html", &Context::new())
    //             .unwrap(),
    //     ),
    // }
}

// PARTIALS:

pub async fn dashboard(auth_session: AuthSession) -> impl IntoResponse {
    Html(TEMPLATES.render("partial/dashboard.html", &Context::new()).unwrap())
}

pub async fn cards(auth_session: AuthSession) -> impl IntoResponse {
    Html(TEMPLATES.render("partial/cards.html", &Context::new()).unwrap())
}

pub async fn schedule(auth_session: AuthSession) -> impl IntoResponse {
    Html(TEMPLATES.render("partial/schedule.html", &Context::new()).unwrap())
}

pub async fn calendar(auth_session: AuthSession) -> impl IntoResponse {
    Html(TEMPLATES.render("partial/calendar.html", &Context::new()).unwrap())
}

