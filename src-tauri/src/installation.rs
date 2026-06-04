use std::fs::{File, create_dir_all};

use tauri::State;

use thiserror::Error;

use serde_json::{to_writer_pretty, json};

use crate::paths::AppPaths;



#[derive(Error, Debug)]
pub enum InstallationError {
    #[error("Couldn't create the folders/files for ATA components")]
    DirectoryCreation(#[from] std::io::Error),
    #[error("Couldn't parse settings file. {0}")]
    Json(#[from] serde_json::Error),
} 



#[tauri::command]
pub fn create_folders(paths: State<AppPaths>) -> Result<(), String> {
    create_folders_inner(&paths).map_err(|er| er.to_string())
}
fn create_folders_inner(paths: &AppPaths) -> Result<(), InstallationError> {
    create_dir_all(&paths.executable)?;
    create_dir_all(&paths.data_file)?;
    create_dir_all(&paths.settings)?;
    create_dir_all(&paths.uis_dir)?;
    create_dir_all(&paths.apps_dir)?;

    Ok(())
}

// fn get_executable() -> Result<(), InstallationError> {
// 
// }

#[tauri::command]
pub fn create_default_data(paths: State<AppPaths>) -> Result<(), String> {
    create_default_data_inner(&paths).map_err(|er| er.to_string())
}
fn create_default_data_inner(paths: &AppPaths) -> Result<(), InstallationError> {
    let data_path = if paths.settings.exists() {
        paths.data_file.parent().unwrap().join("data_valid.json")
    } else {
        paths.settings.clone()
    };

    let data = json!({
        "mods": []
    });
    
    let data_file = File::create(&data_path)?;
    to_writer_pretty(data_file, &data)?;

    Ok(())
}

#[tauri::command]
pub fn create_default_settings(paths: State<AppPaths>) -> Result<(), String> {
    create_default_settings_inner(&paths).map_err(|er| er.to_string())
}
fn create_default_settings_inner(paths: &AppPaths) -> Result<(), InstallationError> {
    let settings_path = if paths.settings.exists() {
        paths.settings.parent().unwrap().join("settings_valid.json")
    } else {
        paths.settings.clone()
    };

    let settings = json!({
        "style": "SilentShulk",
        "palette": "Automata",
        "sortingOrder": "ModType",
        "filesConflictResolution": "Ask",
        "keepExtractedFolders": true,
        "extractedFoldersLocation": "",
        "gamePath": "",
        "discordRichPresence": ""
    });
    
    let settings_file = File::create(&settings_path)?;
    to_writer_pretty(settings_file, &settings)?;

    Ok(())
}