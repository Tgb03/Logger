use std::path::PathBuf;

use directories::ProjectDirs;

pub fn get_directory() -> Option<PathBuf> {
    #[cfg(debug_assertions)]
    if let Some(proj_dirs) = ProjectDirs::from("com", "Tgb03", "GTFO Logger Debug") {
        return Some(proj_dirs.data_dir().to_path_buf());
    }

    #[cfg(not(debug_assertions))]
    if let Some(proj_dirs) = ProjectDirs::from("com", "Tgb03", "GTFO Logger") {
        return Some(proj_dirs.data_dir().to_path_buf());
    }

    None
}

pub fn get_config_directory() -> Option<PathBuf> {
    #[cfg(debug_assertions)]
    if let Some(proj_dirs) = ProjectDirs::from("com", "Tgb03", "GTFO Logger Debug") {
        return Some(proj_dirs.config_dir().to_path_buf());
    }

    #[cfg(not(debug_assertions))]
    if let Some(proj_dirs) = ProjectDirs::from("com", "Tgb03", "GTFO Logger") {
        return Some(proj_dirs.config_dir().to_path_buf());
    }

    None
}
