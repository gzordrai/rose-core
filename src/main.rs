use axum::{Router, extract::State, response::Html, routing::get};
use bollard::Docker;
use std::net::SocketAddr;
use tokio::{net::TcpListener, signal::ctrl_c};
use tracing::info;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::load_config;
use crate::error::Result;

mod config;
mod error;

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello world !</h1>")
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let config = load_config()?;
    let docker = Docker::connect_with_socket_defaults()?;

    let app = Router::new()
        .route("/", get(handler))
        .with_state(docker.clone());

    let ip = config.server.ip;
    let port = config.server.port;
    let addr = SocketAddr::new(ip, port);

    let listener = TcpListener::bind(addr).await?;
    let bound = listener.local_addr()?;

    info!(%bound, "listening");

    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            let _ = ctrl_c().await;
        })
        .await?;

    Ok(())
}
