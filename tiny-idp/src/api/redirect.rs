use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RedirectLoginResponse {
    pub location: String,
}

impl IntoResponse for RedirectLoginResponse {
    fn into_response(self) -> Response {
        (StatusCode::FOUND, self).into_response()
    }
}
