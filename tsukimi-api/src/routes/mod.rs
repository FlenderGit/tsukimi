use crate::AppState;
use serde::{Deserialize, Serialize};

pub(crate) mod engine;

pub fn get_router() -> axum::Router<AppState> {
    axum::Router::new()
        .route(
            "/",
            axum::routing::get(|| async { "Welcome to Tsukimi API!" }),
        )
        .nest("/engine", engine::get_router())
}
