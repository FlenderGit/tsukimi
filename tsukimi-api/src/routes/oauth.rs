use axum::{Router, extract::State};

use crate::AppState;

pub fn get_router() -> Router<AppState> {
    Router::new().route(
        "/github/exchange-code",
        axum::routing::post(exchange_code_handler),
    )
}

async fn exchange_code_handler(
    State(app_state): State<AppState>,
    axum::extract::Json(payload): axum::extract::Json<tsukimi_core::auth::OauthExchangeCodeRequest>,
) -> Result<axum::Json<tsukimi_core::auth::OauthExchangeCodeResponse>, &'static str> {
    let tsukimi_core::auth::OauthExchangeCodeRequest {
        code,
        pkce_code_verifier,
    } = payload;
    let response = app_state
        .oauth
        .exchange_code(code, pkce_code_verifier)
        .await
        .map_err(|_| "Error exchanging code")?;
    Ok(axum::Json(response))
}
