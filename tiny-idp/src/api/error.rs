use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use reqwest::header::HeaderMap;
use std::fmt;

#[derive(Debug)]
pub enum MyError {
    UnauthorizedError(UnauthorizedError),
    ErrorResponse(ErrorResponse),
}

impl IntoResponse for MyError {
    fn into_response(self) -> Response {
        match self {
            MyError::UnauthorizedError(e) => e.into_response(),
            MyError::ErrorResponse(e) => e.into_response(),
        }
    }
}

pub struct AppError(anyhow::Error);

// anyhow::Error => AppError への型変換
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

// AppError => axum::response::Response への型変換
// 自動的に、500 Internal Server Error になるようにハンドリング
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Internal Server Error: {}", self.0),
        )
            .into_response()
    }
}

#[derive(Debug)]
pub struct UnauthorizedError(anyhow::Error);

// anyhow::Error => UnauthorizedError への型変換
impl<E> From<E> for UnauthorizedError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

// UnauthorizedError => axum::response::Response への型変換
// 自動的に、500 Internal Server Error になるようにハンドリング
impl IntoResponse for UnauthorizedError {
    fn into_response(self) -> Response {
        (
            StatusCode::UNAUTHORIZED,
            format!("Unauthorized Error: {}", self.0),
        )
            .into_response()
    }
}

#[derive(Debug, Copy, Clone)]
pub enum TokenError {
    InvalidRequest,
    InvalidClient,
    InvalidGrant,
    UnauthorizedClient,
    UnsupportedGrantType,
    InvalidScope,
}

impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenError::InvalidRequest => write!(f, "invalid_request"),
            TokenError::InvalidClient => write!(f, "invalid_client"),
            TokenError::InvalidGrant => write!(f, "invalid_grant"),
            TokenError::UnauthorizedClient => write!(f, "unauthorized_client"),
            TokenError::UnsupportedGrantType => write!(f, "unsupported_grant_type"),
            TokenError::InvalidScope => write!(f, "invalid_scope"),
        }
    }
}

#[derive(Debug)]
pub struct ErrorResponse {
    pub error: TokenError,
    pub error_description: String,
    pub error_uri: String,
}

impl ErrorResponse {
    pub fn new(error: TokenError) -> Self {
        ErrorResponse {
            error,
            error_description: error.to_string(),
            error_uri: "".to_string(),
        }
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        let body = serde_json::json!({
            "tokenSet": {
            "error": self.error.to_string(),
            }
        });
        let mut header_map = HeaderMap::new();
        header_map.insert("Content-Type", "application/json".parse().unwrap());
        header_map.insert("Cache-Control", "no-store".parse().unwrap());
        header_map.insert("Pragma", "no-cache".parse().unwrap());

        (StatusCode::BAD_REQUEST, header_map, format!("{}", body)).into_response()
    }
}
