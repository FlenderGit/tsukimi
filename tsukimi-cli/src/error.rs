use thiserror::Error;

use crate::{api::ApiError, commands::login::UserInfo};

#[derive(Error, Debug)]
pub enum CliError {
    #[error(transparent)]
    WebError(#[from] ApiError),
    #[error(transparent)]
    CredentialsError(#[from] crate::services::credentials::CredentialsError),
    #[error(transparent)]
    PluginError(#[from] crate::commands::list::PluginError),

    #[error("You are already logged in as {}", .0.format())]
    AlreadyLoggedIn(UserInfo),
}

pub type CliResult = Result<(), CliError>;
