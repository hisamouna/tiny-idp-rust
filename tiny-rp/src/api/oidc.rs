use crate::api::error;
use anyhow::Result;
use axum::{extract::Query, response::Json};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct CallBackQuery {
    code: String,
    scope: String,
}

pub async fn callback(q: Query<CallBackQuery>) -> Result<Json<Value>, error::AppError> {
    let mut map = HashMap::new();
    let code = q.0.code;
    let cloned_code = code.clone();
    map.insert("code", code);
    map.insert(
        "redirect_url",
        "http://localhost:4000/oidc/callback".to_string(),
    );
    map.insert("scope", q.0.scope);
    map.insert("grant_type", "authorization_code".to_string());
    map.insert("client_id", "tiny-client".to_string());

    Ok(Json(serde_json::json!({ "code": cloned_code })))

    //let client = reqwest::Client::new();
    //let url = "http://localhost:3000/openid-connect/token";
    //let resp = client.post(url).json(&map).send().await?;
    //println!("{:?}", resp);
    //let token_set = resp.json::<serde_json::Value>().await?;

    //println!("{:?}", token_set);
    //Ok(Json(token_set))
}
