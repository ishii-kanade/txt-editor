use crate::app::TxtEditorApp;
use crate::file_operations::{get_txt_files_and_dirs_in_directory, move_to_trash};
use eframe::egui::{self, CollapsingHeader, Color32, Context, SidePanel};
use std::fs;
use std::io;
use std::path::PathBuf;

fn display_directory(ui: &mut egui::Ui, path: &PathBuf, app: &mut TxtEditorApp) {
    if path.is_dir() {
        let dir_name = path.file_name().unwrap().to_string_lossy().to_string();
        let is_selected = Some(path) == app.selected_item.as_ref();

        let header = CollapsingHeader::new(dir_name.clone()).default_open(false);

        if is_selected {
            ui.style_mut().visuals.widgets.noninteractive.bg_fill = Color32::YELLOW;
        }

        let response = header.show(ui, |ui| {
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

        // Reset the background color after rendering the header
        if is_selected {
            ui.style_mut().visuals.widgets.noninteractive.bg_fill =
                ui.visuals().widgets.noninteractive.bg_fill;
        }

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
            if ui.button("Rename").clicked() {
                app.rename_popup = true;
                app.rename_target = Some(path.clone());
                // Remove .txt extension for display
                if dir_name.ends_with(".txt") {
                    app.new_name = dir_name.trim_end_matches(".txt").to_string();
                } else {
                    app.new_name = dir_name;
                }
                ui.close_menu();
            }
        });

        if response.header_response.clicked() {
            app.selected_item = Some(path.clone());
            app.selected_file = None; // フォルダを選択した場合、ファイルの選択を解除
        }
    } else {
        let file_name = path.file_name().unwrap().to_string_lossy().to_string();
        if !file_name.starts_with('.') {
            let is_selected = Some(path) == app.selected_item.as_ref();
            let label = if is_selected {
                ui.colored_label(Color32::YELLOW, file_name.clone())
            } else {
                ui.label(file_name.clone())
            };

            if label.clicked() {
                app.selected_item = Some(path.clone());
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
                    app.rename_popup = true;
                    app.rename_target = Some(path.clone());
                    // Remove .txt extension for display
                    if file_name.ends_with(".txt") {
                        app.new_name = file_name.trim_end_matches(".txt").to_string();
                    } else {
                        app.new_name = file_name;
                    }
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

fn rename_item(path: &PathBuf, new_name: &str) -> io::Result<()> {
    let new_name_with_ext = format!("{}.txt", new_name);
    let new_path = path.with_file_name(new_name_with_ext);
    fs::rename(path, new_path)?;
    Ok(())
}

pub fn display(app: &mut TxtEditorApp, ctx: &Context) {
    SidePanel::left("side_panel").show(ctx, |ui| {
        if let Some(ref folder_path) = app.folder_path {
            ui.label(format!("Directory: {}", folder_path.display()));
            ui.separator();

            if app.rename_popup {
                egui::Window::new("Rename").show(ctx, |ui| {
                    ui.label("Enter new name (without extension):");
                    ui.text_edit_singleline(&mut app.new_name);

                    if ui.button("Rename").clicked() {
                        if let Some(ref rename_target) = app.rename_target {
                            if let Err(err) = rename_item(rename_target, &app.new_name) {
                                eprintln!("Failed to rename item: {}", err);
                            } else {
                                if let Some(root_dir) = &app.folder_path {
                                    app.file_list =
                                        get_txt_files_and_dirs_in_directory(root_dir.clone());
                                }
                            }
                        }
                        app.rename_popup = false;
                    }
                    if ui.button("Cancel").clicked() {
                        app.rename_popup = false;
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
