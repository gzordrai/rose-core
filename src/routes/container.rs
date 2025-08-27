use axum::{Json, Router, extract::State, http::StatusCode, routing::get};
use bollard::secret::ContainerSummary;
use bollard::{query_parameters::ListContainersOptionsBuilder, secret::ContainerSummaryStateEnum};
use serde::Serialize;

use super::JsonResponse;
use crate::{error::Result, state::AppState};

#[derive(Debug, Clone, Serialize)]
/// Represents a minimal version of `ContainerSummary` from Bollard, designed to avoid sending unnecessary data in API responses.
pub struct ApiContainerSummary {
    /// The unique identifier of the Docker container.
    pub id: String,

    /// The name of the Docker container.
    pub name: String,

    /// The image used to create the container.
    pub image: String,

    /// The current state of the container (e.g., running, exited).
    pub state: ContainerSummaryStateEnum,

    /// The status description of the container (e.g., "Up 5 minutes").
    pub status: String,

    /// The creation timestamp of the container (Unix time).
    pub created: i64,
}

impl From<ContainerSummary> for ApiContainerSummary {
    /// Converts a `ContainerSummary` from Bollard into an `ApiContainerSummary` for API responses.
    ///
    /// This implementation uses default values for missing fields to ensure that
    /// the API does not fail if a container is not well formatted or has missing data.
    fn from(container: ContainerSummary) -> Self {
        let id = container.id.unwrap_or_default();
        let image = container.image.unwrap_or_default();
        let state = container.state.unwrap_or(ContainerSummaryStateEnum::EMPTY);
        let status = container.status.unwrap_or_default();
        let created = container.created.unwrap_or_default();
        let name = container
            .names
            .and_then(|v| v.into_iter().next())
            .map(|s| s.trim_start_matches('/').to_string())
            .unwrap_or(String::from("unknow"));

        ApiContainerSummary {
            id,
            name,
            image,
            state,
            status,
            created,
        }
    }
}

#[derive(Clone, Serialize)]
/// Represents the response for a list of Docker containers in the API.
pub struct ContainersListResponse {
    /// The list of container summaries returned by the API.
    pub items: Vec<ApiContainerSummary>,

    /// The total number of containers in the response.
    pub count: usize,
}

/// Handles the API request to list Docker containers.
///
/// Fetches all containers using the Docker client, converts them to `ApiContainerSummary`,
/// and returns a JSON response containing the list and count.
async fn containers(State(state): State<AppState>) -> Result<JsonResponse<ContainersListResponse>> {
    let options = ListContainersOptionsBuilder::default().all(true).build();
    let items: Vec<ApiContainerSummary> = state
        .docker
        .list_containers(Some(options))
        .await?
        .into_iter()
        .map(ApiContainerSummary::from)
        .collect();
    let count = items.len();

    Ok((
        StatusCode::OK,
        Json(ContainersListResponse { items, count }),
    ))
}

/// Creates the Axum router for container-related API endpoints.
///
/// # Examples
/// ```
/// use rose_core::routes::container::router;
///
/// let app = router();
/// ```
pub fn router() -> Router<AppState> {
    Router::new().route("/", get(containers))
}
