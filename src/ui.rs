// ui.rs

use crate::app::TxtEditorApp;
use crate::file_operations::{get_txt_files_in_directory, move_to_trash};
use eframe::egui::{
    self, CentralPanel, Color32, Context, Key, Label, Modifiers, Response, RichText, ScrollArea,
    Sense, TextEdit, TopBottomPanel,
};

pub fn display_top_panel(app: &mut TxtEditorApp, ctx: &Context) {
    TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            if ui.button("Select Folder").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    app.folder_path = Some(path.clone());
                    app.file_list = get_txt_files_in_directory(path);
                }
            }

            // Control+A ショートカットをファイル追加に割り当てる
            let add_file_shortcut =
                ctx.input(|i| i.key_pressed(Key::A) && i.modifiers == Modifiers::CTRL);
            if ui.button("Add Text File").clicked() || add_file_shortcut {
                if let Some(folder_path) = &app.folder_path {
                    let new_file_name = "new_file";
                    let new_file_path = folder_path.join(format!("{}.txt", new_file_name));
                    std::fs::File::create(&new_file_path).expect("Failed to create file");
                    app.new_file_popup = true;
                    app.new_file_path = Some(new_file_path);
                    app.new_file_name = new_file_name.to_string(); // Initialize with the default name
                    app.file_list = get_txt_files_in_directory(folder_path.clone());
                    // ファイルリストを更新
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
                            app.file_list = get_txt_files_in_directory(folder_path.clone());
                            // ファイルリストを更新
                        }
                    }
                }
            }

            // 文字数をカウントして表示
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
                        if let Some(folder_path) = &app.folder_path {
                            app.file_list = get_txt_files_in_directory(folder_path.clone());
                            // ファイルリストを更新
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

pub fn display_side_panel(app: &mut TxtEditorApp, ctx: &Context) {
    eframe::egui::SidePanel::left("side_panel").show(ctx, |ui| {
        if let Some(ref folder_path) = app.folder_path {
            ui.label(format!("Directory: {}", folder_path.display()));
            ui.separator();

            // ファイルリストを借用しているスコープを短くするために一時的なベクターを使用
            let files: Vec<_> = app.file_list.iter().cloned().collect();
            for file in files {
                let file_name = file.file_name().unwrap().to_string_lossy();
                let is_selected = Some(&file) == app.selected_file.as_ref();

                let label = if is_selected {
                    ui.colored_label(Color32::YELLOW, file_name)
                } else {
                    ui.label(file_name)
                };

                // 左クリックでファイルを選択
                if label.clicked() {
                    app.selected_file = Some(file.clone());
                    app.file_contents = std::fs::read_to_string(&file)
                        .unwrap_or_else(|_| "Failed to read file".to_string());
                    app.file_modified = false; // ファイルを選択したときは未編集とする
                }

                // 右クリックメニューを追加
                label.context_menu(|ui| {
                    if ui.button("Open in new window").clicked() {
                        // 新しいウィンドウで開く処理をここに追加
                        println!("Open {} in new window", file.display());
                        ui.close_menu();
                    }
                    if ui.button("Rename").clicked() {
                        // リネーム処理をここに追加
                        println!("Rename {}", file.display());
                        ui.close_menu();
                    }
                    if ui.button("Delete").clicked() {
                        if let Err(err) = move_to_trash(&file) {
                            eprintln!("Failed to move file to trash: {}", err);
                        } else {
                            app.file_list.retain(|f| f != &file);
                            if app.selected_file == Some(file.clone()) {
                                app.selected_file = None;
                                app.file_contents.clear();
                            }
                            if let Some(folder_path) = &app.folder_path {
                                app.file_list = get_txt_files_in_directory(folder_path.clone());
                                // ファイルリストを更新
                            }
                        }
                        ui.close_menu();
                    }
                });
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
                        .desired_rows(30) // 固定の行数を設定
                        .desired_width(f32::INFINITY), // 横幅を最大化
                );

                if response.changed() {
                    app.file_modified = true; // テキストが変更されたときにフラグを設定する
                }
            });
        }
    });
}
