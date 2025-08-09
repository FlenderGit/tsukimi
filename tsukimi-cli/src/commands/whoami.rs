use crate::{api, commands::login::get_user, services::credentials::read_token};
use log::info;

pub async fn execute() -> Result<(), api::ApiError> {
    let access_token = read_token().map_err(|_| {
        api::ApiError::AuthenticationError(
            "No access token found. Please log in first.".to_string(),
        )
    })?;
    let user_info = get_user(access_token).await?;
    info!("User info: {:?}", user_info);
    println!("Connected as: {}", user_info.login);
    Ok(())
}
