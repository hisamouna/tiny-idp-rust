mod api;
mod models;

use axum::{http::StatusCode, response::IntoResponse};
use axum::{
    routing::{get, post},
    Router,
};
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_templates=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = router();

    let listener = listener("0.0.0.0", 3000);
    axum::serve(listener.await, app).await.unwrap();
}

fn router() -> Router {
    Router::new()
        .route("/", get(api::root::root)) // Update the route to use the imported module
        .route("/login", post(api::login::login))
        .route("/openid-connect/auth", get(api::html::index))
        .route("/openid-connect/auth", post(api::html::index))
        .route("/openid-connect/token", post(api::token::token))
        .route(
            "/openid-connect/introspect",
            post(api::introspect::introspect),
        )
        .fallback(handler_404)
}

async fn listener(addr: &str, port: u16) -> TcpListener {
    let addr_port = format!("{}:{}", addr, port);
    TcpListener::bind(addr_port).await.unwrap()
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Page not found")
}
