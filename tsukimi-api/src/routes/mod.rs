use crate::AppState;

pub(crate) mod engine;
pub(crate) mod oauth;

pub fn get_router() -> axum::Router<AppState> {
    axum::Router::new()
        .route(
            "/",
            axum::routing::get(|| async { "Welcome to Tsukimi API!" }),
        )
        .nest("/engine", engine::get_router())
        .nest("/oauth", oauth::get_router())
}
