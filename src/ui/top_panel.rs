use crate::app::TxtEditorApp;
use crate::file_operations::{get_txt_files_and_dirs_in_directory, move_to_trash};
use eframe::egui::{self, Context, Key, Modifiers, TopBottomPanel};

pub fn display(app: &mut TxtEditorApp, ctx: &Context) {
    TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            if ui.button("Select Folder").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    app.folder_path = Some(path.clone());
                    app.selected_item = Some(path.clone());
                    app.file_list = get_txt_files_and_dirs_in_directory(path);
                }
            }

            if ui.button("New Folder").clicked() {
                app.new_folder_popup = true;
            }

            if app.new_folder_popup {
                egui::Window::new("Create New Folder").show(ctx, |ui| {
                    ui.label("Enter new folder name:");
                    ui.text_edit_singleline(&mut app.new_folder_name);

                    if ui.button("Create").clicked() {
                        if let Some(selected_dir) = &app.selected_item {
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

            let add_file_shortcut =
                ctx.input(|i| i.key_pressed(Key::A) && i.modifiers == Modifiers::CTRL);
            if ui.button("Add Text File").clicked() || add_file_shortcut {
                if let Some(selected_item) = &app.selected_item {
                    let parent_dir = if selected_item.is_dir() {
                        selected_item
                    } else {
                        selected_item.parent().unwrap_or(selected_item)
                    };
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
            }

            if app.new_file_popup {
                egui::Window::new("Rename New Text File").show(ctx, |ui| {
                    ui.label("Enter new file name:");
                    ui.text_edit_singleline(&mut app.new_file_name);

                    if ui.button("Rename").clicked() {
                        if let Some(ref new_file_path) = app.new_file_path {
                            let parent_dir = new_file_path.parent().unwrap();
                            let new_file_path_renamed =
                                parent_dir.join(format!("{}.txt", app.new_file_name));
                            std::fs::rename(new_file_path, &new_file_path_renamed)
                                .expect("Failed to rename file");
                            app.new_file_popup = false;
                            app.new_file_path = Some(new_file_path_renamed);
                            if let Some(root_dir) = &app.folder_path {
                                app.file_list =
                                    get_txt_files_and_dirs_in_directory(root_dir.clone());
                            }
                        }
                    }
                    if ui.button("Cancel").clicked() {
                        app.new_file_popup = false;
                    }
                });
            }

            let delete_file_shortcut = ctx.input(|i| i.key_pressed(Key::Delete));
            if let Some(ref selected_file) = app.selected_item {
                if !selected_file.is_dir()
                    && (ui.button("Delete").clicked() || delete_file_shortcut)
                {
                    if let Err(err) = move_to_trash(selected_file) {
                        eprintln!("Failed to move file to trash: {}", err);
                    } else {
                        app.file_list.retain(|f| f != selected_file);
                        app.selected_item = None;
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
    });
}
