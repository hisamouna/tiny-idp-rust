use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

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
