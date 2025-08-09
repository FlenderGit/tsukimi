use crate::{api::ApiError, services::credentials::store_token};
use reqwest::header;
use serde::Deserialize;
use tokio::time::{Duration, interval};

static CLIENT_ID: &str = "Iv23li6unikd56cFZ6zX";
static DEVICE_CODE_URL: &str = "https://github.com/login/device/code";
static POLL_URL: &str = "https://github.com/login/oauth/access_token";

#[derive(Debug, Deserialize)]
struct DeviceCodeResponse {
    device_code: String,
    user_code: String,
    verification_uri: String,
    expires_in: u64,
    interval: u64,
}

#[derive(Debug, Deserialize)]
struct AccessTokenResponse {
    access_token: String,
    token_type: String,
    scope: String,
}

#[derive(Debug, Deserialize)]
struct ErrorResponse {
    error: ErrorType,
    error_description: String,
    error_uri: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum ErrorType {
    IncorrectClientCredentials,
    AuthorizationPending,
    SlowDown,
    ExpiredToken,
    UnsupportedGrantType,
    IncorrectDeviceCode,
    AccessDenied,
    DeviceFlowDisabled,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum GithubResponse {
    Success(AccessTokenResponse),
    Error(ErrorResponse),
}

pub async fn execute() -> Result<(), ApiError> {
    let access_token = get_access_token().await?;
    println!("Access token obtained successfully: {}", access_token);

    let user_info = get_user(access_token.to_string()).await?;
    println!("User info: {:?}", user_info);

    store_token(&access_token).map_err(|_| ApiError::Forbidden("".into()))?;
    Ok(())
}

pub async fn get_access_token() -> Result<String, ApiError> {
    let client = reqwest::Client::new();

    let response = client
        .post(DEVICE_CODE_URL)
        .form(&[("client_id", CLIENT_ID)])
        .header("Accept", "application/json")
        .send()
        .await
        .map_err(|e| ApiError::NetworkError(e.to_string()))?;

    if !response.status().is_success() {
        return Err(ApiError::RequestError(
            response.status(),
            response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string()),
        ));
    }

    let device_code_response: DeviceCodeResponse = response
        .json()
        .await
        .map_err(|e| ApiError::ParseError(e.to_string()))?;

    println!("Please open the following URL in your browser to authenticate:");
    println!("{}", device_code_response.verification_uri);
    println!("Enter the user code: {}", device_code_response.user_code);
    println!("Time limit: {} seconds", device_code_response.expires_in);
    println!(
        "Interval for polling: {} seconds",
        device_code_response.interval + 3
    );

    // 2. Polling
    let mut time_interval = device_code_response.interval + 3;
    let mut poll_timer = interval(Duration::from_secs(time_interval));

    loop {
        poll_timer.tick().await;

        let response = client
            .post(POLL_URL)
            .form(&[
                ("client_id", CLIENT_ID),
                ("device_code", &device_code_response.device_code),
                ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
            ])
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| ApiError::NetworkError(e.to_string()))?;

        let text = response
            .text()
            .await
            .map_err(|e| ApiError::ParseError(e.to_string()))?;

        println!("Data received: {}", text);

        let github_response: GithubResponse =
            serde_json::from_str(&text).map_err(|e| ApiError::ParseError(e.to_string()))?;

        match github_response {
            GithubResponse::Success(token_res) => {
                println!("Access Token: {}", token_res.access_token);
                return Ok(token_res.access_token);
            }
            GithubResponse::Error(error_res) => match error_res.error {
                ErrorType::AuthorizationPending => {
                    println!("Authorization pending, retrying...");
                    continue;
                }
                ErrorType::SlowDown => {
                    println!("Told to slow down, increasing wait time...");

                    time_interval += 6; // Augmente le délai d'attente de 5 secondes
                    poll_timer = interval(Duration::from_secs(time_interval));

                    continue;
                }
                ErrorType::ExpiredToken => {
                    return Err(ApiError::AuthenticationError(
                        "Device code expired".to_string(),
                    ));
                }
                _ => {
                    return Err(ApiError::AuthenticationError(format!(
                        "{}: {}",
                        format!("{:?}", error_res.error),
                        error_res.error_description
                    )));
                }
            },
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct GithubUser {
    pub login: String,
    pub id: u64,
    pub avatar_url: String,
    pub html_url: String,
    pub name: Option<String>,
    pub email: Option<String>,
}

pub async fn get_user(access_token: String) -> Result<GithubUser, ApiError> {
    let client = reqwest::Client::new();

    let response = client
        .get("https://api.github.com/user")
        .header(header::AUTHORIZATION, format!("Bearer {}", access_token))
        .header(header::USER_AGENT, "MyRustApp") // GitHub exige un User-Agent
        .send()
        .await
        .map_err(|e| ApiError::NetworkError(e.to_string()))?;

    if !response.status().is_success() {
        return Err(ApiError::RequestError(
            response.status(),
            response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string()),
        ));
    }

    let user: GithubUser = response
        .json()
        .await
        .map_err(|e| ApiError::ParseError(e.to_string()))?;

    Ok(user)
}
