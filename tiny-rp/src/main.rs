mod api;
mod index;

use axum::{routing::get, Router};
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

    let listener = listener("0.0.0.0", 4000);
    axum::serve(listener.await, app).await.unwrap();
}

fn router() -> Router {
    Router::new()
        .route("/hello", get(api::hello::hello))
        .route("/", get(index::index))
        .route("/oidc/callback", get(api::oidc::callback))
}

async fn listener(addr: &str, port: u16) -> TcpListener {
    let addr_port = format!("{}:{}", addr, port);
    TcpListener::bind(addr_port).await.unwrap()
}
