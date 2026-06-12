use std::fs::{File, create_dir_all};

use tauri::State;

use thiserror::Error;

use serde_json::{to_writer_pretty, json};

use crate::paths::Paths;



#[derive(Error, Debug)]
pub enum InstallationError {
    #[error("Couldn't create the folders/files for ATA components")]
    DirectoryCreation(#[from] std::io::Error),
    #[error("Couldn't parse settings file. {0}")]
    Json(#[from] serde_json::Error),
} 



#[tauri::command]
pub fn create_folders(paths: State<Paths>) -> Result<(), String> {
    create_folders_inner(&paths).map_err(|er| er.to_string())
}
fn create_folders_inner(paths: &Paths) -> Result<(), InstallationError> {
    create_dir_all(&paths.executable.parent().unwrap())?;
    create_dir_all(&paths.data_file.parent().unwrap())?;
    create_dir_all(&paths.settings_file.parent().unwrap())?;
    create_dir_all(&paths.uis_dir)?;
    create_dir_all(&paths.apps_dir)?;

    Ok(())
}

// fn get_executable() -> Result<(), InstallationError> {
// 
// }

#[tauri::command]
pub fn create_default_data(paths: State<Paths>) -> Result<(), String> {
    create_default_data_inner(&paths).map_err(|er| er.to_string())
}
fn create_default_data_inner(paths: &Paths) -> Result<(), InstallationError> {
    let data_path = if paths.data_file.exists() {
        paths.data_file.parent().unwrap().join("data_valid.json")
    } else {
        paths.data_file.clone()
    };

    let data = json!({
        "mods": []
    });
    
    let data_file = File::create(&data_path)?;
    to_writer_pretty(data_file, &data)?;

    Ok(())
}

#[tauri::command]
pub fn create_default_settings(paths: State<Paths>) -> Result<(), String> {
    create_default_settings_inner(&paths).map_err(|er| er.to_string())
}
fn create_default_settings_inner(paths: &Paths) -> Result<(), InstallationError> {
    let settings_path = if paths.settings_file.exists() {
        paths.settings_file.parent().unwrap().join("settings_valid.json")
    } else {
        paths.settings_file.clone()
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