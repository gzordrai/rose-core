use std::time::Duration;

use axum::{Json, Router, extract::State, http::StatusCode, routing::get};
use serde::Serialize;
use tokio::time::timeout;

use crate::state::AppState;

const HEALTH_TIMEOUT: Duration = Duration::from_secs(1);

#[derive(Copy, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
enum Status {
    Ok,
    Down,
}

#[derive(Serialize)]
struct HealthResponse {
    status: Status,
    version: String,
    uptime_seconds: u64,
}

async fn health(State(state): State<AppState>) -> (StatusCode, Json<HealthResponse>) {
    let status = match timeout(HEALTH_TIMEOUT, state.docker.ping()).await {
        Ok(Ok(_)) => Status::Ok,
        _ => Status::Down,
    };

    let code = match &status {
        Status::Ok => StatusCode::OK,
        Status::Down => StatusCode::SERVICE_UNAVAILABLE,
    };

    let resp = HealthResponse {
        status: status,
        version: env!("CARGO_PKG_VERSION").into(),
        uptime_seconds: state.start_time.elapsed().as_secs(),
    };

    (code, Json(resp))
}

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(health))
}
