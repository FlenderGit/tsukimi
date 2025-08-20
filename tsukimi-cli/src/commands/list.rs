use directories_next::ProjectDirs;
use log::info;
use reqwest::header::GetAll;
use serde::{Deserialize, Serialize};
use tabled::{Table, Tabled, settings::Style};
use thiserror::Error;
use tsukimi_core::models::Version;

use crate::{error::CliResult, services::project_data::get_project_data_folder};
use std::fmt;

#[derive(Debug)]
enum NotFoundType {
    File(String),
    Directory(String),
    ProjectPath,
}

impl fmt::Display for NotFoundType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NotFoundType::File(name) => write!(f, "file `{}`", name),
            NotFoundType::Directory(name) => write!(f, "directory `{}`", name),
            NotFoundType::ProjectPath => write!(f, "project path"),
        }
    }
}

#[derive(Error, Debug)]
pub enum PluginError {
    #[error("{0} not found")]
    NotFound(NotFoundType),
    #[error("Failed to load plugin: {0}")]
    AlreadyExists(String),
    #[error("Failed to load plugin: {0}")]
    InvalidFormat(String),
    #[error("{0}")]
    ActionFailed(String),
}

#[derive(Tabled, Deserialize)]
pub struct ExtensionManifest {
    pub name: String,
    pub version: Version,
    pub author: String,
    pub repository: String,
}

pub async fn execute() -> CliResult {
    let plugins_dir = get_project_data_folder()
        .ok_or_else(|| PluginError::NotFound(NotFoundType::ProjectPath))?;

    // test if the manifest file exists
    if !plugins_dir.exists() {
        info!(
            "Plugins directory does not exist, creating it at: {}",
            plugins_dir.display()
        );
        std::fs::create_dir_all(&plugins_dir).map_err(|e| {
            PluginError::ActionFailed(format!("Failed to create plugins directory: {}", e))
        })?;
    }

    // test if the manifest file exists
    let manifest_path = plugins_dir.join("manifest.json");
    if !manifest_path.exists() {
        return Err(PluginError::NotFound(NotFoundType::File("manifest.json".to_string())).into());
    }

    // Load the manifest file
    let manifest_content = std::fs::read_to_string(&manifest_path)
        .map_err(|e| PluginError::ActionFailed(format!("Failed to read manifest file: {}", e)))?;

    let list: Vec<ExtensionManifest> = serde_json::from_str(&manifest_content)
        .map_err(|e| PluginError::InvalidFormat(format!("Failed to parse manifest file: {}", e)))?;

    let mut table = Table::new(list);
    table.with(Style::rounded());
    println!("{table}");

    Ok(())
}
