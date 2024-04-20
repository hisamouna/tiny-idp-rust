use crate::api::error;
use crate::models::access_token::AccessToken;
use crate::models::context::USERS;
use axum::{
    http::StatusCode,
    response::{Json, Response},
    Form,
};
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct PostTokenParam {
    token: String,
}

pub async fn introspect(Form(param): Form<PostTokenParam>) -> Result<Json<Value>, error::MyError> {
    let pram = param.token;

    let token = match find_token(&pram) {
        Some(token) => token,
        None => {
            return Err(error::MyError::UnauthorizedError(
                error::UnauthorizedError::from(anyhow::anyhow!("invalid token")),
            ));
        }
    };
    if token.expires_at < chrono::Utc::now().naive_local().date() {
        return Err(error::MyError::UnauthorizedError(
            error::UnauthorizedError::from(anyhow::anyhow!("expired token")),
        ));
    }

    let builder = Response::builder();
    let body = Json(serde_json::json!({
        "active": true
    }));

    Ok(builder
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(body)
        .unwrap()
        .into_body())
}

fn find_token(token: &str) -> Option<AccessToken> {
    USERS
        .lock()
        .unwrap()
        .access_tokens
        .iter()
        .find(|at| at.token == token)
        .cloned()
}
