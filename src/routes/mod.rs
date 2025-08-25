use axum::{Json, Router, http::StatusCode, response::IntoResponse};

use crate::state::AppState;

pub type Response<T> = (StatusCode, T);
pub type JsonResponse<T> = Response<Json<T>>;

pub mod health;

pub fn router() -> Router<AppState> {
    Router::new().nest("/health", health::router())
}

pub async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}
