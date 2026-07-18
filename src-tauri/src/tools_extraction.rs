use std::fs::File;
use std::io::Write;
use tauri::State;

use crate::paths::Paths;

#[cfg(target_os = "linux")]
const ATA: &[u8] = include_bytes!("../embedded/ATA-x86_64-unknown-linux-gnu");
#[cfg(target_os = "windows")]
const ATA: &[u8] = include_bytes!("../embedded/ATA-x86_64-pc-windows-msvc.exe");

#[cfg(target_os = "linux")]
const SHELLUI: &[u8] = include_bytes!("../embedded/shellUI-x86_64-unknown-linux-gnu");
#[cfg(target_os = "windows")]
const SHELLUI: &[u8] = include_bytes!("../embedded/shellUI-x86_64-pc-windows-msvc.exe");

#[tauri::command]
pub fn extract_tools(paths: State<Paths>) -> Result<(), String> {
    extract_tools_inner(&paths).map_err(|er| er.to_string())
}

fn extract_tools_inner(paths: &Paths) -> Result<(), std::io::Error> {
    let mut ata = File::create(&paths.executable)?;
    let mut shellui = File::create(&paths.apps_dir.join("ShellUI"))?;

    ata.write_all(ATA)?;
    shellui.write_all(SHELLUI)?;

    #[cfg(unix)] {
        use std::fs::{set_permissions, Permissions};
        use std::os::unix::fs::PermissionsExt;
        set_permissions(&paths.executable, Permissions::from_mode(0o755))?;
        set_permissions(&paths.apps_dir.join("ShellUI"), Permissions::from_mode(0o755))?;
    }
    
    Ok(())    
}