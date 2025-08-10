use crate::AppState;
use crate::services::database::ApiPagination;
use axum::extract::{Query, State};
use tsukimi_core::models::Engine;

pub fn get_router() -> axum::Router<AppState> {
    axum::Router::new().route("/", axum::routing::get(get_engines))
}

async fn get_engines(
    Query(pagination): Query<ApiPagination>,
    State(app_state): State<AppState>,
) -> Result<axum::Json<Vec<Engine>>, &'static str> {
    let list = app_state
        .database
        .get_engines(pagination)
        .await
        .map_err(|_| "Error fetching engines")?;
    Ok(axum::Json(list))
}
