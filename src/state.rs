use bollard::Docker;
use tokio::time::Instant;

use crate::config::Config;

#[derive(Debug, Clone)]
pub struct AppState {
    pub config: Config,
    pub docker: Docker,
    pub start_time: Instant,
}

impl AppState {
    pub fn new(config: Config, docker: Docker) -> Self {
        AppState {
            config,
            docker,
            start_time: Instant::now(),
        }
    }
}
