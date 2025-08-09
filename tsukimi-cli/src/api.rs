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
