use crate::app::TxtEditorApp;
use crate::file_operations::{get_txt_files_and_dirs_in_directory, move_to_trash};
use eframe::egui::{
    self, CentralPanel, CollapsingHeader, Color32, Context, Key, Modifiers, ScrollArea, TextEdit,
    TopBottomPanel,
};
use std::fs;
use std::path::PathBuf;

pub fn display_top_panel(app: &mut TxtEditorApp, ctx: &Context) {
    TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            if ui.button("Select Folder").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    app.folder_path = Some(path.clone());
                    app.selected_dir = Some(path.clone());
                    app.file_list = get_txt_files_and_dirs_in_directory(path);
                }
            }

            let add_file_shortcut =
                ctx.input(|i| i.key_pressed(Key::A) && i.modifiers == Modifiers::CTRL);
            if ui.button("Add Text File").clicked() || add_file_shortcut {
                if let Some(selected_file) = &app.selected_file {
                    let parent_dir = selected_file.parent().unwrap_or(selected_file);
                    let new_file_name = "new_file";
                    let new_file_path = parent_dir.join(format!("{}.txt", new_file_name));
                    std::fs::File::create(&new_file_path).expect("Failed to create file");
                    app.new_file_popup = true;
                    app.new_file_path = Some(new_file_path);
                    app.new_file_name = new_file_name.to_string();
                    if let Some(root_dir) = &app.folder_path {
                        app.file_list = get_txt_files_and_dirs_in_directory(root_dir.clone());
                    }
                } else if let Some(selected_dir) = &app.selected_dir {
                    let new_file_name = "new_file";
                    let new_file_path = selected_dir.join(format!("{}.txt", new_file_name));
                    std::fs::File::create(&new_file_path).expect("Failed to create file");
                    app.new_file_popup = true;
                    app.new_file_path = Some(new_file_path);
                    app.new_file_name = new_file_name.to_string();
                    if let Some(root_dir) = &app.folder_path {
                        app.file_list = get_txt_files_and_dirs_in_directory(root_dir.clone());
                    }
                }
            }

            let delete_file_shortcut = ctx.input(|i| i.key_pressed(Key::Delete));
            if let Some(ref selected_file) = app.selected_file {
                if ui.button("Delete").clicked() || delete_file_shortcut {
                    if let Err(err) = move_to_trash(selected_file) {
                        eprintln!("Failed to move file to trash: {}", err);
                    } else {
                        app.file_list.retain(|f| f != selected_file);
                        app.selected_file = None;
                        app.file_contents.clear();
                        if let Some(folder_path) = &app.folder_path {
                            app.file_list =
                                get_txt_files_and_dirs_in_directory(folder_path.clone());
                        }
                    }
                }
            }

            let char_count = app.file_contents.chars().count();
            ui.label(format!("Character count: {}", char_count));
        });

        if app.new_file_popup {
            egui::Window::new("Rename File").show(ctx, |ui| {
                ui.label("Enter new file name (without extension):");
                ui.text_edit_singleline(&mut app.new_file_name);

                if ui.button("Rename").clicked() {
                    if let Some(new_file_path) = &app.new_file_path {
                        let parent_dir = new_file_path
                            .parent()
                            .expect("Failed to get parent directory");
                        let new_path = parent_dir.join(format!("{}.txt", app.new_file_name));
                        std::fs::rename(new_file_path, &new_path).expect("Failed to rename file");
                        app.file_list.pop();
                        app.file_list.push(new_path);
                        if let Some(root_dir) = &app.folder_path {
                            app.file_list = get_txt_files_and_dirs_in_directory(root_dir.clone());
                        }
                    }
                    app.new_file_popup = false;
                }

                if ui.button("Cancel").clicked() {
                    app.new_file_popup = false;
                }
            });
        }
    });
}

fn display_directory(ui: &mut egui::Ui, path: &PathBuf, app: &mut TxtEditorApp) {
    if path.is_dir() {
        let dir_name = path.file_name().unwrap().to_string_lossy().to_string();
        let response = CollapsingHeader::new(dir_name.clone())
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

pub fn display_side_panel(app: &mut TxtEditorApp, ctx: &Context) {
    eframe::egui::SidePanel::left("side_panel").show(ctx, |ui| {
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

pub fn display_central_panel(app: &mut TxtEditorApp, ctx: &Context) {
    CentralPanel::default().show(ctx, |ui| {
        if let Some(_) = app.selected_file {
            ScrollArea::vertical().show(ui, |ui| {
                let response = ui.add(
                    TextEdit::multiline(&mut app.file_contents)
                        .font(egui::TextStyle::Monospace)
                        .desired_rows(30)
                        .desired_width(f32::INFINITY),
                );

                if response.changed() {
                    app.file_modified = true;
                }
            });
        }
    });
}

pub fn display_right_panel(app: &mut TxtEditorApp, ctx: &Context) {
    eframe::egui::SidePanel::right("right_panel").show(ctx, |ui| {
        if let Some(_) = app.right_panel_file {
            ScrollArea::vertical().show(ui, |ui| {
                ui.add(
                    TextEdit::multiline(&mut app.right_panel_contents)
                        .font(egui::TextStyle::Monospace)
                        .desired_rows(30)
                        .desired_width(f32::INFINITY),
                );
            });
        }
    });
}
