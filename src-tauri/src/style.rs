use std::fs::{read_dir, read_to_string, File};

use tauri::State;

use thiserror::Error;

use serde_json::{Value, from_str, to_writer_pretty};

use crate::paths::Paths;



#[derive(Error, Debug)]
pub enum StyleError {
    #[error("Couldn't read settings file. {0}")]
    Io(#[from] std::io::Error),
    #[error("Couldn't parse settings file. {0}")]
    Json(#[from] serde_json::Error),
}



#[tauri::command]
pub fn scan_for_styles(paths: State<Paths>) -> Result<Vec<String>, String> {
    scan_for_styles_inner(&paths).map_err(|er| er.to_string())
}

fn scan_for_styles_inner(paths: &Paths) -> Result<Vec<String>, StyleError> {
    let mut styles: Vec<String> = Vec::new();

    for entry in std::fs::read_dir(&paths.uis_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(stem) = path.file_stem() {
                let s = stem.to_string_lossy().into_owned();
                styles.push(s);
            }
        }
    }

    for entry in read_dir(&paths.apps_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(stem) = path.file_stem() {
                let s = stem.to_string_lossy().into_owned();
                styles.push(s);
            }
        }
    }

    Ok(styles)
}

#[tauri::command]
pub fn get_selected_style(paths: State<Paths>) -> Result<String, String> {
    get_selected_style_inner(&paths).map_err(|e| e.to_string())
}
fn get_selected_style_inner(paths: &Paths) -> Result<String, StyleError> {
    let contents = read_to_string(&paths.settings_file)?;
    let settings: Value = from_str(&contents)?;
    Ok(settings["style"].as_str().unwrap_or("").to_string())
}

#[tauri::command]
pub fn set_selected_style(selected_style: String, paths: State<Paths>) -> Result<(), String> {
    set_selected_style_inner(selected_style, &paths).map_err(|er| er.to_string())
}
fn set_selected_style_inner(selected_style: String, paths: &Paths) -> Result<(), StyleError> {
    let contents = read_to_string(&paths.settings_file)?;
    let mut settings: Value = from_str(&contents)?;

    settings["style"] = Value::String(selected_style);
    
    let settings_file = File::create(&paths.settings_file)?;
    to_writer_pretty(settings_file, &settings)?;

    Ok(())
}