use crate::config::GithubConfiguration;
use oauth2::{
    AuthUrl, AuthorizationCode, Client, ClientId, ClientSecret, EmptyExtraTokenFields,
    EndpointNotSet, EndpointSet, PkceCodeVerifier, RevocationErrorResponseType,
    StandardErrorResponse, StandardRevocableToken, StandardTokenIntrospectionResponse,
    StandardTokenResponse, TokenResponse, TokenUrl,
    basic::{BasicClient, BasicErrorResponseType, BasicTokenType},
};
use tracing::error;

#[derive(Debug, Clone)]
pub struct OAuthService {
    pub http_client: reqwest::Client,
    pub client: Client<
        StandardErrorResponse<BasicErrorResponseType>,
        StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
        StandardTokenIntrospectionResponse<EmptyExtraTokenFields, BasicTokenType>,
        StandardRevocableToken,
        StandardErrorResponse<RevocationErrorResponseType>,
        EndpointNotSet,
        EndpointNotSet,
        EndpointNotSet,
        EndpointNotSet,
        EndpointSet,
    >,
}

impl TryFrom<&GithubConfiguration> for OAuthService {
    type Error = String;

    fn try_from(github_config: &GithubConfiguration) -> Result<Self, Self::Error> {
        let client = BasicClient::new(ClientId::new(github_config.client_id().to_string()))
            .set_client_secret(ClientSecret::new(github_config.client_secret().to_string()))
            .set_token_uri(
                TokenUrl::new("https://github.com/login/oauth/access_token".to_string())
                    .map_err(|e| e.to_string())?,
            );
        let http_client = reqwest::Client::new();
        Ok(OAuthService {
            http_client,
            client,
        })
    }
}

impl OAuthService {
    // pub async fn get_authorization_url(&self) -> Result<String, oauth2::RequestTokenError> {
    //     let (authorize_url, _csrf_state) = self.client.authorize_url(oauth2::CsrfToken::new_random);
    //     Ok(authorize_url.to_string())
    // }

    pub async fn exchange_code(
        &self,
        code: AuthorizationCode,
        pkce_code_verifier: PkceCodeVerifier,
    ) -> Result<tsukimi_core::auth::OauthExchangeCodeResponse, String> {
        let token_result = self
            .client
            .exchange_code(code)
            .set_pkce_verifier(pkce_code_verifier)
            .request_async(&self.http_client)
            .await
            .map_err(|e| {
                error!("Failed to exchange code for token: {:?}", e);
                e.to_string()
            })?;
        Ok(tsukimi_core::auth::OauthExchangeCodeResponse {
            access_token: token_result.access_token().to_owned(),
            refresh_token: token_result.refresh_token().map(|s| s.to_owned()),
            expires_in: token_result.expires_in(),
            scopes: token_result.scopes().map(|s| s.to_owned()),
        })
    }
}
