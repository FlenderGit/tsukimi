use crate::{
    api::ApiError,
    error::{CliError, CliResult},
    services::credentials::{read_token, store_token},
};
use inquire::Select;
use log::info;
use oauth2::{
    AccessToken, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge,
    RedirectUrl, RefreshToken, Scope, TokenResponse, TokenUrl, basic::BasicClient,
};
use reqwest::{Url, header};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tokio::{
    io::AsyncBufReadExt,
    io::AsyncWriteExt,
    io::BufReader,
    net::TcpListener,
    time::{Duration, interval},
};

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
    access_token: AccessToken,
    token_type: String,
    scope: Scope,
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
enum DafPollResponse {
    Success(AccessTokenResponse),
    Error(ErrorResponse),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Provider {
    DeviceCode,
    OAuth,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthSession {
    pub provider: Provider,
    pub access_token: AccessToken,
    pub refresh_token: Option<RefreshToken>,
    pub expires_at: Option<OffsetDateTime>,
    pub scopes: Option<Vec<Scope>>,
}

impl AuthSession {
    pub async fn fetch_user(&self) -> Result<UserInfo, ApiError> {
        let client = reqwest::Client::new();
        let response = client
            .get("https://api.github.com/user")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", self.access_token.secret()),
            )
            .header(header::USER_AGENT, "MyRustApp") // GitHub exige un User-Agent
            .send()
            .await
            .map_err(|e| ApiError::NetworkError(e.to_string()))?;

        let user_info: UserInfo = response
            .json()
            .await
            .map_err(|e| ApiError::ParseError(e.to_string()))?;

        Ok(user_info)
    }
}

impl std::fmt::Display for Provider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.str_select())
    }
}

impl Provider {
    fn str_select(&self) -> &str {
        match self {
            Provider::DeviceCode => "Login with a web browser (Device Code)",
            Provider::OAuth => "Login with OAuth (not implemented yet)",
        }
    }
}

pub async fn execute() -> CliResult {
    // Check if the user is already logged in
    if let Ok(session) = read_token() {
        let user_info = session.fetch_user().await?;
        return Err(CliError::AlreadyLoggedIn(user_info));
    }

    // Select the authentication method (Device Code or OAuth)
    let auth_method = Select::new(
        "Choose an authentication method:",
        vec![Provider::DeviceCode, Provider::OAuth],
    )
    .with_formatter(&|item| item.value.str_select().to_string())
    .with_vim_mode(true)
    .prompt()
    .map_err(|_| ApiError::AuthenticationError("Failed to select authentication method".into()))?;

    let session = match auth_method {
        Provider::DeviceCode => device_flow_get_access_token().await?,
        Provider::OAuth => oauth2_get_access_token().await?,
    };
    println!("Authentification complete.");
    info!("Receive access token: {}", session.access_token.secret());

    store_token(&session)?;

    let user_info = session.fetch_user().await?;
    println!("Connected as: {}", user_info.format());

    Ok(())
}

async fn oauth2_get_access_token() -> Result<AuthSession, ApiError> {
    let client = BasicClient::new(ClientId::new(CLIENT_ID.to_string()))
        .set_auth_uri(AuthUrl::new("https://github.com/login/oauth/authorize".to_string()).unwrap())
        .set_token_uri(TokenUrl::new(POLL_URL.to_string()).unwrap())
        .set_redirect_uri(RedirectUrl::new("http://localhost:7777/callback".to_string()).unwrap());

    let http_client = reqwest::ClientBuilder::new()
        // Following redirects opens the client up to SSRF vulnerabilities.
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Client should build");

    let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

    let (authorize_url, csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        .set_pkce_challenge(pkce_code_challenge)
        // This example is requesting access to the user's public repos and email.
        .add_scope(Scope::new("public_repo".to_string()))
        .add_scope(Scope::new("user:email".to_string()))
        .url();

    if let Err(_) = open::that(authorize_url.to_string()) {
        println!("Failed to open the browser. Please open the following URL manually:");
        println!("{}", authorize_url);
    }

    let (code, _state) = {
        let listener = TcpListener::bind("127.0.0.1:7777")
            .await
            .map_err(|e| ApiError::NetworkError(e.to_string()))?;
        println!("Waiting for the user to authenticate...");

        loop {
            if let Ok((mut stream, _)) = listener.accept().await {
                let mut reader = BufReader::new(&mut stream);

                let mut request_line = String::new();
                reader.read_line(&mut request_line).await.unwrap();

                let redirect_url = request_line.split_whitespace().nth(1).unwrap();
                let url = Url::parse(&("http://localhost".to_string() + redirect_url)).unwrap();

                let code = url
                    .query_pairs()
                    .find(|(key, _)| key == "code")
                    .map(|(_, code)| AuthorizationCode::new(code.into_owned()))
                    .unwrap();

                let state = url
                    .query_pairs()
                    .find(|(key, _)| key == "state")
                    .map(|(_, state)| CsrfToken::new(state.into_owned()))
                    .unwrap();

                let message = include_str!("../res/oauth2_connected.html");
                let response = format!(
                    "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                    message.len(),
                    message
                );
                stream.write_all(response.as_bytes()).await.unwrap();

                // The server will terminate itself after collecting the first code.
                break (code, state);
            }
        }
    };

    info!("Received code: {}", code.secret());

    let response = http_client
        .post("http://localhost:3000/oauth/github/exchange-code")
        .json(&tsukimi_core::auth::OauthExchangeCodeRequest {
            code,
            pkce_code_verifier,
        })
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

    let token_response: tsukimi_core::auth::OauthExchangeCodeResponse = response
        .json()
        .await
        .map_err(|e| ApiError::ParseError(e.to_string()))?;

    Ok(AuthSession {
        provider: Provider::OAuth,
        access_token: token_response.access_token,
        refresh_token: token_response.refresh_token,
        expires_at: token_response
            .expires_in
            .map(|it| OffsetDateTime::now_utc() + it),
        scopes: token_response.scopes,
    })
}

async fn device_flow_get_access_token() -> Result<AuthSession, ApiError> {
    let client = reqwest::Client::new();

    let response = client
        .post(DEVICE_CODE_URL)
        .form(&[
            ("client_id", CLIENT_ID),
            // ("scope", "read:user")
        ])
        .header("Accept", "application/json")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(ApiError::NetworkError("".to_string()));
    }

    let device_code_response: DeviceCodeResponse = response.json().await?;

    let _ = open::that(&device_code_response.verification_uri);

    println!("Please open the following URL in your browser to authenticate:");
    println!("{}", device_code_response.verification_uri.clone());
    println!("Enter the user code: {}", device_code_response.user_code);
    println!("Time limit: {} seconds", device_code_response.expires_in);
    println!(
        "Interval for polling: {} seconds",
        device_code_response.interval + 1
    );

    // 2. Polling
    let mut time_interval = device_code_response.interval + 1;
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
            .await?;

        if !response.status().is_success()
            && response.status() != reqwest::StatusCode::PRECONDITION_REQUIRED
        {
            return Err(ApiError::RequestError(
                response.status(),
                response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string()),
            ));
        }

        let text = response
            .text()
            .await
            .map_err(|e| ApiError::ParseError(e.to_string()))?;

        println!("Data received: {}", text);

        let github_response: DafPollResponse =
            serde_json::from_str(&text).map_err(|e| ApiError::ParseError(e.to_string()))?;

        match github_response {
            DafPollResponse::Success(token_res) => {
                println!("Access Token: {}", token_res.access_token.secret());
                return Ok(AuthSession {
                    provider: Provider::DeviceCode,
                    access_token: token_res.access_token,
                    refresh_token: None,
                    expires_at: None,
                    scopes: Some(
                        token_res
                            .scope
                            .split(' ')
                            .map(|it| Scope::new(it.to_string()))
                            .collect(),
                    ),
                });
            }
            DafPollResponse::Error(error_res) => match error_res.error {
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
pub struct UserInfo {
    pub login: String,
    pub id: u64,
    pub avatar_url: String,
    pub html_url: String,
    pub name: Option<String>,
    pub email: Option<String>,
}

impl UserInfo {
    pub fn format(&self) -> String {
        let name = match &self.name {
            Some(name) => name.clone(),
            None => self.login.clone(),
        };
        let email = self.email.clone().unwrap_or_else(|| "No email".to_string());
        format!("{} <{}>", name, email)
    }
}

// pub async fn get_access_token() -> Result<String, ApiError> {
//     let device_auth_url = DeviceAuthorizationUrl::new(DEVICE_CODE_URL.to_string())
//         .expect("Invalid device authorization URL");
//     let client = BasicClient::new(ClientId::new(CLIENT_ID.to_string()))
//         .set_device_authorization_url(device_auth_url)
//         .set_token_uri(TokenUrl::new(POLL_URL.to_string()).expect("Invalid token URL"));

//     let http_client = reqwest::Client::builder()
//         .default_headers({
//             let mut headers = reqwest::header::HeaderMap::new();
//             headers.insert(header::ACCEPT, "application/json".parse().unwrap());
//             headers
//         })
//         .build()
//         .expect("Failed to build HTTP client");

//     let details: StandardDeviceAuthorizationResponse = client
//         .exchange_device_code()
//         .request_async(&http_client)
//         .await
//         .map_err(|e| ApiError::NetworkError(e.to_string()))?;

//     println!("Please open the following URL in your browser to authenticate:");
//     println!("{}", details.verification_uri());
//     if let Some(complete) = details.verification_uri_complete() {
//         println!("Complete URL: {}", complete.secret());
//     } else {
//         println!("Verification URI complete is not available.");
//     }
//     println!("Enter the user code: {}", details.user_code().secret());

//     let access_token = client
//         .exchange_device_access_token(&details)
//         .request_async(
//             // &http_client,
//             |req| github_device_token_http_adapter(req, &client),
//             tokio::time::sleep,
//             None,
//         )
//         .await
//         .map_err(|e| {
//             error!("Failed to exchange device access token: {:?}", e);
//             ApiError::NetworkError(e.to_string())
//         })?;

//     Ok(access_token.access_token().secret().to_string())
// }
// pub async fn github_device_token_http_adapter(
//     request: HttpRequest,
//     client: &reqwest::Client,
// ) -> Result<HttpResponse, Box<dyn std::error::Error>> {
//     // 1. Utilise le client pour envoyer la requête
//     let reqwest_req = reqwest::Request::try_from(request.clone())?; // dépend de ton mapping
//     let resp: Response = client.execute(reqwest_req).await?;

//     // 2. Copie le body
//     let status = resp.status();
//     let headers = resp.headers().clone();
//     let body = resp.bytes().await?;

//     // 3. Si HTTP 200 ET body contient "error", patch le status
//     if status == StatusCode::OK {
//         if let Ok(json) = serde_json::from_slice::<serde_json::Value>(&body) {
//             if json.get("error").is_some() {
//                 // Patch le status à BAD_REQUEST
//                 return Ok(HttpResponse::new(StatusCode::BAD_REQUEST, headers, body));
//             }
//         }
//     }

//     // Sinon, retourne la réponse originale
//     Ok(HttpResponse::new(status, headers, body))
// }
