use axum::{Json, Router, http::StatusCode};

use crate::state::AppState;

pub type Response<T> = (StatusCode, T);
pub type JsonResponse<T> = Response<Json<T>>;

pub mod health;

pub fn router() -> Router<AppState> {
    Router::new().nest("/health", health::router())
}
