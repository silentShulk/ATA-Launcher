use std::fs::{read_dir, read_to_string};

use tauri::State;

use thiserror::Error;

use serde_json::{Value, from_str};

use crate::paths::AppPaths;



#[derive(Error, Debug)]
pub enum StyleError {
    #[error("Couldn't read settings file. {0}")]
    Io(#[from] std::io::Error),
    #[error("Couldn't parse settings file. {0}")]
    Json(#[from] serde_json::Error),
}



#[tauri::command]
pub fn scan_for_styles(paths: State<AppPaths>) -> Result<Vec<String>, String> {
    scan_for_styles_inner(&paths).map_err(|er| er.to_string())
}

fn scan_for_styles_inner(paths: &AppPaths) -> Result<Vec<String>, StyleError> {
    let mut styles: Vec<String> = Vec::new();

    for entry in read_dir(&paths.uis_dir)? {
        let entry = entry?;
        if entry.path().is_file() {
            styles.push(entry.file_name().to_string_lossy().into_owned());
        }
    }

    for entry in read_dir(&paths.apps_dir)? {
        let entry = entry?;
        if entry.path().is_file() {
            styles.push(entry.file_name().to_string_lossy().into_owned());
        }
    }

    Ok(styles)
}

#[tauri::command]
pub fn get_selected_style(paths: State<AppPaths>) -> Result<String, String> {
    get_selected_style_inner(&paths).map_err(|e| e.to_string())
}

fn get_selected_style_inner(paths: &AppPaths) -> Result<String, StyleError> {
    let contents = read_to_string(&paths.settings)?;
    let settings: Value = from_str(&contents)?;
    Ok(settings["style"].as_str().unwrap_or("").to_string())
}