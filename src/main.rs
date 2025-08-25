use bollard::Docker;
use std::net::SocketAddr;
use tokio::{net::TcpListener, signal::ctrl_c};
use tracing::info;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::load_config;
use crate::error::Result;
use crate::state::AppState;

mod config;
mod error;
mod routes;
mod state;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        // .with(EnvFilter::from_default_env())
        .init();

    let config = load_config()?;
    let docker = Docker::connect_with_socket_defaults()?;

    // Read ip and port before moving config into state
    let ip = config.server.ip;
    let port = config.server.port;
    let state = AppState::new(config, docker);

    let app = routes::router()
        .with_state(state)
        .fallback(routes::handler_404);

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
