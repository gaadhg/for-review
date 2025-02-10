use axum::{
    Form,
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
};

use crate::{domain::Student, infrastructure::*};

pub async fn login(
    mut auth_session: AuthSession,
    Form(creds): Form<Credentials>,
) -> impl IntoResponse {
    let user = match auth_session.authenticate(creds.clone()).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return Redirect::to("/app").into_response();
        }
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };
    match auth_session.login(&user).await {
        Ok(_) => {}
        Err(e) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }

    Redirect::to("/app").into_response()
}

pub async fn register(
    mut auth_session: AuthSession,
    Form(creds): Form<Credentials>,
) -> impl IntoResponse {
    if let Ok(None) = auth_session.backend.get_student_by_email(&creds.email) {
        auth_session
            .backend
            .update_student(Student::new(&creds.email, &creds.password, None).unwrap())
            .unwrap();
        return Redirect::to("/app").into_response();
    }

    StatusCode::INTERNAL_SERVER_ERROR.into_response()
}

pub async fn logout(mut auth_session: AuthSession) -> impl IntoResponse {
    match auth_session.logout().await {
        Ok(_) => Redirect::to("/app").into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
