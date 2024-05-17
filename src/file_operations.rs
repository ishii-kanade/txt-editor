use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub fn get_txt_files_in_directory(path: PathBuf) -> Vec<PathBuf> {
    fs::read_dir(path)
        .unwrap()
        .filter_map(Result::ok)
        .filter(|entry| {
            entry
                .path()
                .extension()
                .map(|ext| ext == "txt")
                .unwrap_or(false)
        })
        .map(|entry| entry.path())
        .collect()
}

pub fn move_to_trash(path: &PathBuf) -> Result<(), String> {
    let path_str = path
        .to_str()
        .ok_or_else(|| "Failed to convert path to string".to_string())?;

    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args(&["/C", "move", path_str, "%USERPROFILE%\\Recycle Bin"])
            .output()
            .map_err(|e| format!("Failed to move file to trash: {}", e))?;
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("osascript")
            .args(&[
                "-e",
                &format!(
                    "tell application \"Finder\" to delete POSIX file \"{}\"",
                    path_str
                ),
            ])
            .output()
            .map_err(|e| format!("Failed to move file to trash: {}", e))?;
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("gio")
            .args(&["trash", path_str])
            .output()
            .map_err(|e| format!("Failed to move file to trash: {}", e))?;
    }

    Ok(())
}
