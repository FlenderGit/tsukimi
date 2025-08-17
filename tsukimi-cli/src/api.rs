use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("Request error: {0} - {1}")]
    RequestError(reqwest::StatusCode, String),
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("Authentication error: {0}")]
    AuthenticationError(String),
    #[error("Authorization error: {0}")]
    NotFound(String),
    #[error("Internal server error: {0}")]
    InternalServerError(String),
    #[error("Service unavailable: {0}")]
    Unauthorized(String),
    #[error("Forbidden: {0}")]
    Forbidden(String),
    #[error("Bad request: {0}")]
    BadRequest(String),
}

impl From<reqwest::Error> for ApiError {
    fn from(err: reqwest::Error) -> Self {
        if let Some(status) = err.status() {
            ApiError::RequestError(status, err.to_string())
        } else {
            ApiError::NetworkError(err.to_string())
        }
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(err: serde_json::Error) -> Self {
        ApiError::ParseError(err.to_string())
    }
}
