use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde_json::json;

pub async fn hello() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({"message": "Hello, world!"})))
}
