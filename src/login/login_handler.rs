use askama::Template;
use axum::{extract::State, response::IntoResponse, Form};
use axum_login::{AuthUser, UserStore};
use axum_sessions::extractors::WritableSession;
use serde::{ Deserialize};

use crate::{
    login::auth_user::BlogAuthor,
    utils::common::{AppState, HtmlTemplate, MyAuthContext},
};

#[derive(Deserialize, Debug, Clone)]
pub struct SignInForm {
    pub username: String,
    pub password: String,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct Success {}

pub async fn signin_handler(
    mut auth: MyAuthContext,
    mut session: WritableSession,
    State(app_state): State<AppState>,
    Form(input): Form<SignInForm>,
) -> impl IntoResponse {
    let i = &input;
    dbg!(i);
    //auth.login()
    // DO BETTER validation
    let _u = BlogAuthor {
        username: input.username,
        password_hash: String::from(""),
    };
    let id = _u.get_id();
    let user = app_state
        .userstore
        .load_user(id.as_str())
        .await
        .unwrap()
        .unwrap();
    let u = &user;
    dbg!(u);
    let _ = auth.login(&user);
    dbg!(user);
    HtmlTemplate(Success {})
}
