mod checks;
use checks::check_installation_state;
use checks::get_selected_ui;



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            check_installation_state,
            get_selected_ui,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
