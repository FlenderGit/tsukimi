use inquire::Text;
use log::info;

use crate::{
    error::CliResult,
    services::{api::ApiService, project_data::get_local_extension_state},
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
    let extension_local = get_local_extension_state(&engine_name);

    // Fetch online the engine by name
    let engine = ApiService::default().fetch_engine(&engine_name).await?;

    // Compare the versions
    // If the online version is newer, download it
    // If the online version is the same, do nothing
    let need_install = match extension_local {
        None => {
            info!("Engine `{}` not found locally, downloading...", engine_name);
            true
        }
        Some(value) if value.version > engine.current_version => {
            info!(
                "Engine `{}` is outdated (local: {}, online: {}), updating...",
                engine_name, value.version, engine.current_version
            );
            true
        }
        Some(value) if value.version == engine.current_version => {
            info!(
                "Engine `{}` is up to date (version: {}). No action needed.",
                engine_name, value.version
            );
            false
        }
        Some(value) if value.version < engine.current_version => {
            if params.force.unwrap_or(false) {
                info!(
                    "Engine `{}` is outdated (local: {}, online: {}). Forcing update...",
                    engine_name, value.version, engine.current_version
                );
                true
            } else {
                info!(
                    "Engine `{}` is outdated (local: {}, online: {}). Use --force to update.",
                    engine_name, value.version, engine.current_version
                );
                false
            }
        }
        Some(_) => {
            info!(
                "Engine `{}` is already installed with the latest version.",
                engine_name
            );
            false
        }
    };

    Ok(())
}
