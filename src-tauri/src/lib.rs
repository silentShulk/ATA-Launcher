mod checks;
mod installation;
mod paths;
mod style;

use checks::check_installation_state;
use installation::{create_default_data, create_default_settings, create_folders};
use paths::Paths;
use style::{get_selected_style, scan_for_styles, set_selected_style};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .manage(Paths::new())
        .invoke_handler(tauri::generate_handler![
            check_installation_state,
            scan_for_styles,
            get_selected_style,
            set_selected_style,
            create_folders,
            create_default_data,
            create_default_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
