pub mod auth;
pub mod error;
pub mod extractors;
pub mod user;

use std::sync::Arc;

use axum::{response::Html, Extension, Router};
use sqlx::PgPool;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use crate::config::Config;

#[derive(Clone)]
pub struct ApiContext {
    pub config: Arc<Config>,
    pub db: PgPool,
}

impl ApiContext {
    pub fn new(db: PgPool, config: Config) -> ApiContext {
        let config = Arc::new(config);
        ApiContext { config, db }
    }
}

pub fn fallback404() -> Html<&'static str> {
    Html("<h1>Resource not found, if you're seeing blame frontend</h1>")
}

pub async fn serve(config: Config, db: PgPool) -> anyhow::Result<()> {
    let app = api_router()
        .layer(
            ServiceBuilder::new()
                .layer(Extension(ApiContext::new(db, config)))
                // Enables logging. Use `RUST_LOG=tower_http=debug`
                .layer(TraceLayer::new_for_http()),
        )
        .fallback(fallback404());

    // listening globally in order to be able to interact it trough our localhost when in docker
    // let listener = TcpListener::bind("127.0.0.1:3000")
    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Could not bind to port");
    tracing::debug!(
        "listening on {}",
        listener.local_addr().expect("This should not fail")
    );
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

fn api_router() -> Router {
    // This is the order that the modules were authored in.
    user::router().merge(auth::router())
}
