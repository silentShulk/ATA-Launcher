use std::path::PathBuf;
use dirs::{home_dir, data_dir, config_local_dir, state_dir, data_local_dir};

pub struct AppPaths {
    pub executable: PathBuf,
    pub data_file:  PathBuf,
    pub settings:   PathBuf,
    pub uis_dir:    PathBuf,
    pub apps_dir:   PathBuf,
}

impl AppPaths {
    pub fn new() -> Self {
        #[cfg(target_os = "linux")]
        return Self {
            executable: home_dir().unwrap()
                .join(".local").join("bin").join("ATA").join("ATA"),
            data_file: data_dir().unwrap()
                .join("ATA").join("data.json"),
            settings: config_local_dir().unwrap()
                .join("ATA").join("settings.json"),
            uis_dir: state_dir().or_else(data_local_dir).unwrap()
                .join("ATA").join("UIs"),
            apps_dir: state_dir().or_else(data_local_dir).unwrap()
                .join("ATA").join("Apps"),
        };

        #[cfg(target_os = "windows")]
        return Self {
            executable: data_local_dir().unwrap()
                .join("Programs").join("ATA").join("ATA.exe"),
            data_file: data_dir().unwrap()
                .join("ATA").join("data.json"),
            settings: config_local_dir().unwrap()
                .join("ATA").join("settings.json"),
            uis_dir: data_local_dir().unwrap()
                .join("ATA").join("UIs"),
            apps_dir: data_local_dir().unwrap()
                .join("ATA").join("Apps"),
        };

        #[cfg(not(any(target_os = "linux", target_os = "windows")))]
        compile_error!("ATA only supports Linux and Windows");
    }
}