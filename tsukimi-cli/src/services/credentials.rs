use log::info;

use crate::commands::login::AuthSession;

static SERVICE_NAME: &str = "tsukimi";
static USERNAME: &str = "github_access_token";

#[derive(Debug, thiserror::Error)]
pub enum CredentialsError {
    #[error("Failed to store token")]
    TokenNotFound,
    #[error("Failed to read token")]
    TokenReadError,
}

impl From<keyring::Error> for CredentialsError {
    fn from(err: keyring::Error) -> Self {
        match err {
            keyring::Error::NoEntry => CredentialsError::TokenNotFound,
            _ => CredentialsError::TokenNotFound, // Handle other errors as needed
        }
    }
}

pub fn store_token(session: &AuthSession) -> Result<(), CredentialsError> {
    info!("Storing token in keyring...");
    let store = keyring::Entry::new(SERVICE_NAME, USERNAME)?;
    let session_str =
        serde_json::to_string(session).map_err(|_| CredentialsError::TokenNotFound)?;
    store.set_password(session_str.as_str())?;
    info!("Token stored successfully.");
    Ok(())
}

pub fn read_token() -> Result<AuthSession, CredentialsError> {
    info!("Reading token from keyring...");
    let entry = keyring::Entry::new(SERVICE_NAME, USERNAME)?;
    let token = entry.get_password()?;
    info!("Token read successfully.");

    let session: AuthSession =
        serde_json::from_str(&token).map_err(|_| CredentialsError::TokenNotFound)?;
    Ok(session)
}

pub fn delete_token() -> Result<(), CredentialsError> {
    info!("Deleting token from keyring...");
    let store = keyring::Entry::new(SERVICE_NAME, USERNAME)?;
    store.delete_credential()?;
    info!("Token deleted successfully.");
    Ok(())
}
