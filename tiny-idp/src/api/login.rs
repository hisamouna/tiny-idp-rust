use crate::api::error;
use crate::api::redirect::RedirectLoginResponse;
use crate::models::auth_code;
use crate::models::context::USERS;
use crate::models::user::User;
use anyhow::Result;
use axum::response::Redirect;
use axum::{extract::Query, Form};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginQuery {
    redirect_uri: Option<String>,
    scope: Option<String>,
    client_id: Option<String>,
}

#[derive(Deserialize)]
pub struct LoginParam {
    email: String,
    password: String,
}

pub async fn login(
    q: Query<LoginQuery>,
    Form(param): Form<LoginParam>,
) -> Result<Redirect, error::UnauthorizedError> {
    let email = param.email;
    let password = param.password;
    let redirect_uri = q.0.redirect_uri.unwrap_or("".to_string());
    println!(
        "(api.login) email: {}, password: {}, redirect_uri: {}",
        email, password, redirect_uri
    );

    let scope = q.0.scope.unwrap_or("".to_string());
    let client_id = q.0.client_id.unwrap_or("".to_string());
    let issuer = "http://localhost:3000";

    if !email.is_empty()
        && !password.is_empty()
        && User::login(&USERS.lock().unwrap().users, &email, &password)
    {
        let user = find_by_email(&email)?;
        let user_id = user.id.to_string();
        let auth_code = auth_code::build(user_id, client_id, &redirect_uri);
        let code = auth_code.code.clone();
        auth_code.save();
        //return Ok(RedirectLoginResponse {
        //    location: format!(
        //        "{}?code={}&iss={}&scope={}",
        //        &redirect_uri, code, issuer, scope
        //    ),
        //});
        return Ok(Redirect::to(&format!(
            "{}?code={}&iss={}&scope={}",
            &redirect_uri, code, issuer, scope
        )));
    }
    Err(error::UnauthorizedError::from(anyhow::anyhow!(
        "Unauthorized"
    )))
}

fn find_by_email(email: &str) -> Result<User, error::UnauthorizedError> {
    let us = &USERS.lock().unwrap();
    match User::find_by_email(&us.users, email) {
        Some(user) => Ok(user.clone()),
        None => Err(error::UnauthorizedError::from(anyhow::anyhow!(
            "Unauthorized"
        ))),
    }
}
