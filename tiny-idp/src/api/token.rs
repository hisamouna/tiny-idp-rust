use crate::api::error;
use crate::models::access_token::AccessToken;
use crate::models::auth_code::AuthCode;
use crate::models::context::USERS;
use axum::{
    http::StatusCode,
    response::{Json, Response},
    Form,
};
use chrono::Utc;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct PostTokenParam {
    grant_type: Option<String>,
    code: String,
    redirect_uri: Option<String>,
    client_id: String,
    client_secret: String,
}

pub async fn token(Form(param): Form<PostTokenParam>) -> Result<Json<Value>, error::MyError> {
    if let Err(error) = validate(&param) {
        return Err(error::MyError::ErrorResponse(error::ErrorResponse::new(
            error,
        )));
    }

    let code = param.code;
    let client_id = param.client_id;
    //let client_secret = param.client_secret;
    //let grant_type = param.grant_type.unwrap_or("".to_string());
    //let redirect_uri = param.redirect_uri.unwrap_or("".to_string());
    println!(
        "Token ------- > code: {:?}, cliend_id: {:?}",
        &code, &client_id
    );
    let auth_code = match get_autocode(&code, &client_id) {
        Some(auth_code) => auth_code,
        None => {
            return Err(error::MyError::UnauthorizedError(
                error::UnauthorizedError::from(anyhow::anyhow!("invalid code")),
            ));
        }
    };
    let user_id = auth_code.user_id.clone();
    auth_code.save();

    let at = AccessToken::build(user_id);
    let token = at.token.clone();
    at.save();

    let builder = Response::builder();
    let body = Json(serde_json::json!({
        "tokenSet": {
            "id_token": "dummy-id-token",
            "access_token": token,
            "token_type": "Bearer",
            "expires_in": 86400,
        }
    }));

    Ok(builder
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .header("Cache-Control", "no-store")
        .header("Pragma", "no-cache")
        .body(body)
        .unwrap()
        .into_body())
}

fn get_autocode(code: &str, client_id: &str) -> Option<AuthCode> {
    let auth_code = match USERS.lock().unwrap().auth_codes.iter().find(|auth_code| {
        auth_code.code == code && auth_code.client_id == client_id
        //&& auth_code.expires_at > Utc::now().naive_local().date()
    }) {
        Some(auth_code) => {
            let ac = auth_code.clone();
            AuthCode {
                code: ac.code,
                user_id: ac.user_id,
                client_id: ac.client_id,
                expires_at: ac.expires_at,
                used_at: Some(Utc::now().naive_local().date()),
                redirect_uri: ac.redirect_uri,
            }
        }
        None => {
            return None;
        }
    };
    Some(auth_code)
}

fn validate(param: &PostTokenParam) -> Result<(), error::TokenError> {
    println!("Validate -------{:?}", &param);
    if param.grant_type != Some("authorization_code".to_string()) {
        return Err(error::TokenError::UnsupportedGrantType);
    }
    if param.client_id.is_empty() || param.code.is_empty() {
        return Err(error::TokenError::InvalidRequest);
    }
    if USERS.lock().unwrap().auth_codes.is_empty()
        || USERS.lock().unwrap().auth_codes.iter().any(|auth_code| {
            auth_code.used_at.is_some()
                || (param.redirect_uri.is_some()
                    && &auth_code.redirect_uri != param.redirect_uri.as_ref().unwrap())
        })
    {
        return Err(error::TokenError::InvalidGrant);
    }

    if USERS.lock().unwrap().clients.is_empty()
        || USERS
            .lock()
            .unwrap()
            .clients
            .iter()
            .any(|client| client.client_secret != param.client_secret)
    {
        return Err(error::TokenError::InvalidClient);
    }
    Ok(())
}
