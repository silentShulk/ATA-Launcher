use tauri::State;
use crate::paths::AppPaths;

#[tauri::command]
pub fn check_installation_state(paths: State<AppPaths>) -> (bool, bool, bool, bool) {
    let folders  = are_folders_present(&paths);
    let ata      = is_executable_present(&paths);
    let data     = is_data_present(&paths);
    let settings = is_settings_present(&paths);

    (folders, ata, data, settings)
}

fn are_folders_present(paths: &AppPaths) -> bool {
    paths.data_file.parent().map_or(false, |p| p.exists())
        && paths.settings.parent().map_or(false, |p| p.exists())
        && paths.uis_dir.exists()
        && paths.apps_dir.exists()
        && paths.executable.parent().map_or(false, |p| p.exists())
}

fn is_executable_present(paths: &AppPaths) -> bool {
    paths.executable.exists()
}

fn is_data_present(paths: &AppPaths) -> bool {
    paths.data_file.exists()
}

fn is_settings_present(paths: &AppPaths) -> bool {
    paths.settings.exists()
}