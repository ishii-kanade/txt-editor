use crate::app::TxtEditorApp;
use crate::file_operations::get_txt_files_and_dirs_in_directory;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

pub fn add_text_file(app: &mut TxtEditorApp, parent_dir: &Path) {
    let new_file_name = "new_file";
    let new_file_path = parent_dir.join(format!("{}.txt", new_file_name));
    std::fs::File::create(&new_file_path).expect("Failed to create file");
    app.new_file_popup = true;
    app.new_file_path = Some(new_file_path);
    app.new_file_name = new_file_name.to_string();
    if let Some(root_dir) = &app.folder_path {
        app.file_list = get_txt_files_and_dirs_in_directory(root_dir.clone());
    }
}

pub fn create_folder(parent_dir: &PathBuf, folder_name: &str) -> io::Result<()> {
    let new_folder_path = parent_dir.join(folder_name);
    fs::create_dir(new_folder_path)?;
    Ok(())
}
