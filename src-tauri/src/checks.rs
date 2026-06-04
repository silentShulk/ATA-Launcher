use dirs;
use serde::Serialize;



#[derive(Serialize)]
pub enum InstallationState {
    #[serde(rename = "Nothing is Installed")]
    NothingInstalled,
    #[serde(rename = "Executable is Missing")]
    MissingExecutable,
    #[serde(rename = "Data is Missing")]
    MissingData,
    #[serde(rename = "Settings are Missing")]
    MissingSettings,
    #[serde(rename = "Executable and Data are Missing")]
    MissingExecutableData,
    #[serde(rename = "Executable and Settings are Missing")]
    MissingExecutableSettings,
    #[serde(rename = "Data and Settings are Missing")]
    MissingDataSettings,
    #[serde(rename = "Everything is Installed")]    
    EverythingInstalled,
}

#[tauri::command]
pub fn check_installation_state() -> InstallationState {
    let ata = is_executable_present();
    let data = is_data_present();
    let settings = is_settings_present();
    
    match (ata, data, settings) {
        (false, false, false) => InstallationState::NothingInstalled,
        (false, true, true) => InstallationState::MissingExecutable,
        (true, false, true) => InstallationState::MissingData,
        (true, true, false) => InstallationState::MissingSettings,
        (false, false, true) => InstallationState::MissingExecutableData,
        (false, true, false) => InstallationState::MissingExecutableSettings,
        (true, false, false) => InstallationState::MissingDataSettings,
        (true, true, true) => InstallationState::EverythingInstalled,
    }
}

fn is_executable_present() -> bool {
    let ata_path = if cfg!(target_os = "linux") {
        dirs::home_dir().unwrap().join(".local").join("bin").join("ATA").join("ATA")
    } else if cfg!(target_os = "windows") {
        dirs::home_dir().unwrap().join("AppData").join("Local").join("ATA").join("ATA.exe")
    } else {
        return false;
    };
    
    ata_path.exists()
}
fn is_data_present() -> bool {
    let data_path = dirs::data_local_dir().unwrap().join("ATA").join("data.json");
    
    data_path.exists()
}
fn is_settings_present() -> bool {
    let settings_path = dirs::config_local_dir().unwrap().join("ATA").join("settings.json");
    
    settings_path.exists()
}

#[tauri::command]
pub fn get_selected_ui() -> String {
    let settings_path = dirs::config_local_dir().unwrap().join("ATA").join("settings.json");
    
    let contents = std::fs::read_to_string(&settings_path).unwrap();
    let settings: serde_json::Value = serde_json::from_str(&contents).unwrap();
    
    settings["style"].as_str().unwrap_or("").to_string()
}
