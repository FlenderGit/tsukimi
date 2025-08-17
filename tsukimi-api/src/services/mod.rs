use crate::config::Configuration;

pub mod database;
pub mod oauth;

pub async fn get_services(
    config: &Configuration,
) -> Result<(database::DatabaseService, oauth::OAuthService), String> {
    let database_service = database::DatabaseService::new(&config.database().connection_string())
        .await
        .map_err(|e| format!("Failed to create database service: {}", e))?;

    let github_config = config.github();
    let oauth_service = github_config
        .try_into()
        .map_err(|e| format!("Failed to create OAuth service: {}", e))?;

    Ok((database_service, oauth_service))
}
