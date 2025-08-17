use crate::{error::CliResult, services::credentials::delete_token};

pub async fn execute() -> CliResult {
    let _ = delete_token()?;
    Ok(())
}
