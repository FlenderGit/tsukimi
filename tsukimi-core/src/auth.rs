use std::time::Duration;

use oauth2::{AccessToken, AuthorizationCode, PkceCodeVerifier, RefreshToken, Scope};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct OauthExchangeCodeRequest {
    pub code: AuthorizationCode,
    pub pkce_code_verifier: PkceCodeVerifier,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OauthExchangeCodeResponse {
    pub access_token: AccessToken,
    pub refresh_token: Option<RefreshToken>,
    pub expires_in: Option<Duration>,
    pub scopes: Option<Vec<Scope>>,
}
