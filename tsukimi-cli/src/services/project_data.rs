use std::path::PathBuf;

use directories_next::ProjectDirs;

fn get_project_folder() -> Option<ProjectDirs> {
    ProjectDirs::from("com", "flender", "tsukimi")
}

pub fn get_project_data_folder() -> Option<PathBuf> {
    get_project_folder().map(|dirs| dirs.data_dir().to_path_buf())
}

// pub fn get_manifest()

pub fn get_local_extension_state(name: &str) -> bool {
    false
}
