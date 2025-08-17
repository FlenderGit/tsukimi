use crate::{error::CliResult, services::credentials::read_token};
use log::{error, info};

pub async fn execute() -> CliResult {
    let session = read_token().map_err(|e| {
        error!("Failed to read access token: {}", e);
        e
    })?;
    let user_info = session.fetch_user().await?;
    info!("User info: {:?}", user_info);
    println!("Connected as: {}", user_info.format());
    Ok(())
}
