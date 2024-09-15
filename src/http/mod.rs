use std::{net::SocketAddr, sync::Arc};

use axum::{extract::FromRef, Extension, Router};
use sqlx::PgPool;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use crate::config::{self, Config};

pub mod auth;
pub mod error;
pub mod user;

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


pub async fn serve(config: Config, db: PgPool) -> anyhow::Result<()> {
    let app = api_router().layer(
        ServiceBuilder::new()
            .layer(Extension(ApiContext::new(db, config)))
            // Enables logging. Use `RUST_LOG=tower_http=debug`
            .layer(TraceLayer::new_for_http()),
    );

    // Define the socket address directly
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

fn api_router() -> Router {
    // This is the order that the modules were authored in.
    user::router().merge(auth::router())
}
