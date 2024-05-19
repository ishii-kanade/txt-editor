use crate::app::TxtEditorApp;
use crate::file_operations::{get_txt_files_and_dirs_in_directory, move_to_trash};
use eframe::egui::{self, CollapsingHeader, Color32, Context, SidePanel};
use std::fs;
use std::path::PathBuf;

fn display_directory(ui: &mut egui::Ui, path: &PathBuf, app: &mut TxtEditorApp) {
    if path.is_dir() {
        let dir_name = path.file_name().unwrap().to_string_lossy().to_string();
        let is_selected = Some(path) == app.selected_dir.as_ref();

        let response = CollapsingHeader::new(if is_selected {
            format!("[{}]", dir_name)
        } else {
            dir_name.clone()
        })
        .default_open(false)
        .show(ui, |ui| {
            if let Ok(entries) = fs::read_dir(path) {
                for entry in entries.flatten() {
                    let entry_path = entry.path();
                    if let Some(file_name) = entry_path.file_name() {
                        if !file_name.to_string_lossy().starts_with('.') {
                            display_directory(ui, &entry_path, app);
                        }
                    }
                }
            }
        });

        response.header_response.context_menu(|ui| {
            if ui.button("Delete Directory").clicked() {
                if let Err(err) = move_to_trash(path) {
                    eprintln!("Failed to move directory to trash: {}", err);
                } else {
                    app.file_list.retain(|f| f != path);
                    if let Some(root_dir) = &app.folder_path {
                        app.file_list = get_txt_files_and_dirs_in_directory(root_dir.clone());
                    }
                }
                ui.close_menu();
            }
        });

        if response.header_response.clicked() {
            app.selected_dir = Some(path.clone());
        }
    } else {
        let file_name = path.file_name().unwrap().to_string_lossy().to_string();
        if !file_name.starts_with('.') {
            let is_selected = Some(path) == app.selected_file.as_ref();
            let label = if is_selected {
                ui.colored_label(Color32::YELLOW, file_name)
            } else {
                ui.label(file_name)
            };

            if label.clicked() {
                app.selected_file = Some(path.clone());
                app.file_contents = std::fs::read_to_string(&path)
                    .unwrap_or_else(|_| "Failed to read file".to_string());
                app.file_modified = false;
            }

            label.context_menu(|ui| {
                if ui.button("Open in RightPanel").clicked() {
                    app.right_panel_file = Some(path.clone());
                    app.right_panel_contents = std::fs::read_to_string(&path)
                        .unwrap_or_else(|_| "Failed to read file".to_string());
                    ui.close_menu();
                }
                if ui.button("Rename").clicked() {
                    println!("Rename {}", path.display());
                    ui.close_menu();
                }
                if ui.button("Delete").clicked() {
                    if let Err(err) = move_to_trash(&path) {
                        eprintln!("Failed to move file to trash: {}", err);
                    } else {
                        app.file_list.retain(|f| f != path);
                        if app.selected_file == Some(path.clone()) {
                            app.selected_file = None;
                            app.file_contents.clear();
                        }
                        if let Some(root_dir) = &app.folder_path {
                            app.file_list = get_txt_files_and_dirs_in_directory(root_dir.clone());
                        }
                    }
                    ui.close_menu();
                }
            });
        }
    }
}

pub fn display(app: &mut TxtEditorApp, ctx: &Context) {
    SidePanel::left("side_panel").show(ctx, |ui| {
        if let Some(ref folder_path) = app.folder_path {
            ui.label(format!("Directory: {}", folder_path.display()));
            ui.separator();

            if ui.button("New Folder").clicked() {
                app.new_folder_popup = true;
            }

            if app.new_folder_popup {
                egui::Window::new("Create New Folder").show(ctx, |ui| {
                    ui.label("Enter new folder name:");
                    ui.text_edit_singleline(&mut app.new_folder_name);

                    if ui.button("Create").clicked() {
                        if let Some(selected_dir) = &app.selected_dir {
                            let new_folder_path = selected_dir.join(&app.new_folder_name);
                            if let Err(err) = std::fs::create_dir(&new_folder_path) {
                                eprintln!("Failed to create folder: {}", err);
                            } else {
                                if let Some(root_dir) = &app.folder_path {
                                    app.file_list =
                                        get_txt_files_and_dirs_in_directory(root_dir.clone());
                                }
                            }
                        }
                        app.new_folder_popup = false;
                    }
                    if ui.button("Cancel").clicked() {
                        app.new_folder_popup = false;
                    }
                });
            }

            let paths = app.file_list.clone();
            for path in paths {
                display_directory(ui, &path, app);
            }
        }
    });
}
