use crate::api::error;
use anyhow::Result;
use axum::{extract::Query, response::Json};
use dioxus::html::p;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct CallBackQuery {
    code: String,
    scope: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TokenRequestBody {
    code: String,
    scope: String,
    grant_type: String,
    client_id: String,
    client_secret: String,
    redirect_url: String,
}

pub async fn callback(q: Query<CallBackQuery>) -> Result<Json<Value>, error::AppError> {
    let trb = TokenRequestBody {
        code: q.0.code,
        scope: q.0.scope,
        grant_type: "authorization_code".to_string(),
        client_id: "tiny-client".to_string(),
        client_secret: "c1!3n753cr37".to_string(),
        redirect_url: "http://localhost:4000/oidc/callback".to_string(),
    };
    let body = serde_urlencoded::to_string(&trb).unwrap();
    println!("BODY>>>>>> {:?}", body);

    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/x-www-form-urlencoded"),
    );

    let client = reqwest::Client::new();
    let url = "http://localhost:3000/openid-connect/token";
    let resp = client.post(url).headers(headers).body(body).send().await?;
    println!("RESPONSE>>>>>> {:?}", resp);
    let token_set = resp.json::<serde_json::Value>().await?;

    println!("{:?}", token_set);
    Ok(Json(token_set))
}
