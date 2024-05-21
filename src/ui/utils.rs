use crate::app::TxtEditorApp;
use crate::file_operations::get_txt_files_and_dirs_in_directory;
use std::path::Path;

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
