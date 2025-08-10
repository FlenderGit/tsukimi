use crate::config::Configuration;

pub mod database;

pub async fn get_services(config: &Configuration) -> Result<(database::DatabaseService), String> {
    let database_service = database::DatabaseService::new(&config.database().connection_string())
        .await
        .map_err(|e| format!("Failed to create database service: {}", e))?;

    Ok((database_service))
}
