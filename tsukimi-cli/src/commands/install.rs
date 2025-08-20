use inquire::Text;
use tsukimi_core::models::Version;

use crate::{
    error::CliResult,
    services::{api::ApiService, project_data::is_extension_downloaded},
};

#[derive(clap::Args)]
pub struct InstallCommandParams {
    engine: Option<String>,
    force: Option<bool>,
}

pub async fn execute(params: InstallCommandParams) -> CliResult {
    let engine_name = params.engine.unwrap_or_else(|| {
        Text::new("Enter the engine name to install:")
            .prompt()
            .unwrap()
    });

    // Search if file exists in local data
    let exists_local = get_local_extension_state(&engine_name);

    // Fetch online the engine by name
    let engine = ApiService::default().fetch_engine(&engine_name).await?;

    // Compare the versions
    // If the online version is newer, download it
    // If the online version is the same, do nothing
    match engine.current_version < {

    }


    Ok(())
}
