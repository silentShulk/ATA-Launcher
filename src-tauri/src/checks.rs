use crate::paths::Paths;
use tauri::State;

#[tauri::command]
pub fn check_installation_state(paths: State<Paths>) -> (bool, bool, bool, bool) {
    let folders = are_folders_present(&paths);
    let ata = is_executable_present(&paths);
    let data = is_data_present(&paths);
    let settings = is_settings_present(&paths);

    (folders, ata, data, settings)
}

fn are_folders_present(paths: &Paths) -> bool {
    paths.data_file.parent().map_or(false, |p| p.exists())
        && paths.settings_file.parent().map_or(false, |p| p.exists())
        && paths.uis_dir.exists()
        && paths.apps_dir.exists()
        && paths.executable.parent().map_or(false, |p| p.exists())
}

fn is_executable_present(paths: &Paths) -> bool {
    paths.executable.exists()
}

fn is_data_present(paths: &Paths) -> bool {
    paths.data_file.exists()
}

fn is_settings_present(paths: &Paths) -> bool {
    paths.settings_file.exists()
}
