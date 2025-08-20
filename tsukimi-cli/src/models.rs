use serde::Deserialize;
use tabled::Tabled;
use tsukimi_core::models::Version;

#[derive(Tabled, Deserialize)]
pub struct ExtensionManifest {
    pub name: String,
    pub version: Version,
    pub author: String,
    pub repository: String,
}
