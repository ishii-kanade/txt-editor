use crate::app::TxtEditorApp;
use crate::file_operations::{get_txt_files_and_dirs_in_directory, move_to_trash};
use crate::ui::utils::{add_text_file, create_folder};
use eframe::egui::{self, CollapsingHeader, Color32, Context, SidePanel};
use std::fs;
use std::io;
use std::path::PathBuf;

// フォルダの表示
fn display_directory(ui: &mut egui::Ui, path: &PathBuf, app: &mut TxtEditorApp) {
    if path.is_dir() {
        let dir_name = path.file_name().unwrap().to_string_lossy().to_string();
        let is_selected = Some(path) == app.selected_item.as_ref();

        if is_selected {
            ui.style_mut().visuals.widgets.noninteractive.bg_fill = Color32::YELLOW;
        }

        let header = CollapsingHeader::new(dir_name.clone()).default_open(false);
        let response = header.show(ui, |ui| display_entries(ui, path, app));

        if is_selected {
            ui.style_mut().visuals.widgets.noninteractive.bg_fill =
                ui.visuals().widgets.noninteractive.bg_fill;
        }

        if response.header_response.clicked() {
            app.selected_item = Some(path.clone());
            app.selected_file = None;
        }
    } else {
        display_file(ui, path, app);
    }
}

// フォルダ内のエントリ表示
fn display_entries(ui: &mut egui::Ui, path: &PathBuf, app: &mut TxtEditorApp) {
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
}

// ファイルの表示
fn display_file(ui: &mut egui::Ui, path: &PathBuf, app: &mut TxtEditorApp) {
    let file_name = path.file_name().unwrap().to_string_lossy().to_string();
    if !file_name.starts_with('.') {
        let is_selected = Some(path) == app.selected_item.as_ref();
        let label = if is_selected {
            ui.colored_label(Color32::YELLOW, file_name.clone())
        } else {
            ui.label(file_name.clone())
        };

        let response = label;

        if response.clicked() {
            select_file(app, path);
        }

        response.context_menu(|ui| {
            if ui.button("Open in RightPanel").clicked() {
                app.right_panel_file = Some(path.clone());
                app.right_panel_contents = std::fs::read_to_string(path)
                    .unwrap_or_else(|_| "Failed to read file".to_string());
                ui.close_menu();
            }
            if ui.button("Rename").clicked() {
                app.rename_popup = true;
                app.rename_target = Some(path.clone());
                app.new_name = if file_name.ends_with(".txt") {
                    file_name.trim_end_matches(".txt").to_string()
                } else {
                    file_name.to_string()
                };
                ui.close_menu();
            }
            if ui.button("Delete").clicked() {
                delete_file(app, path);
                ui.close_menu();
            }
            if ui.button("Add Text File").clicked() {
                add_text_file_to_selected_directory(app);
                ui.close_menu();
            }
            if ui.button("Add Folder").clicked() {
                add_folder_to_selected_directory(app);
                ui.close_menu();
            }
        });
    }
}

// ファイルの選択
fn select_file(app: &mut TxtEditorApp, path: &PathBuf) {
    app.selected_item = Some(path.clone());
    app.selected_file = Some(path.clone());
    app.file_contents =
        std::fs::read_to_string(path).unwrap_or_else(|_| "Failed to read file".to_string());
    app.file_modified = false;
}

// ファイルの削除
fn delete_file(app: &mut TxtEditorApp, path: &PathBuf) {
    if let Err(err) = move_to_trash(path) {
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
}

// テキストファイルの追加
fn add_text_file_to_selected_directory(app: &mut TxtEditorApp) {
    if let Some(selected_item) = &app.selected_item {
        let parent_dir = if selected_item.is_dir() {
            selected_item.clone()
        } else {
            selected_item
                .parent()
                .unwrap_or(selected_item)
                .to_path_buf()
        };
        add_text_file(app, &parent_dir);
    }
}

// フォルダの追加
fn add_folder_to_selected_directory(app: &mut TxtEditorApp) {
    if let Some(selected_item) = &app.selected_item {
        let parent_dir = if selected_item.is_dir() {
            selected_item.clone()
        } else {
            selected_item
                .parent()
                .unwrap_or(selected_item)
                .to_path_buf()
        };
        app.new_folder_popup = true;
        app.new_folder_parent = Some(parent_dir);
    }
}

// 名前の変更
fn rename_item(path: &PathBuf, new_name: &str) -> io::Result<()> {
    let new_name_with_ext = if path.is_file() {
        format!("{}.txt", new_name)
    } else {
        new_name.to_string()
    };
    let new_path = path.with_file_name(new_name_with_ext);
    fs::rename(path, new_path)?;
    Ok(())
}

// メインの表示
pub fn display(app: &mut TxtEditorApp, ctx: &Context) {
    SidePanel::left("side_panel").show(ctx, |ui| {
        if let Some(ref folder_path) = app.folder_path {
            ui.label(format!("Directory: {}", folder_path.display()));
            ui.separator();

            if app.rename_popup {
                rename_popup(ui, ctx, app);
            }

            if app.new_folder_popup {
                new_folder_popup(ui, ctx, app);
            }

            let paths = app.file_list.clone();
            for path in paths {
                display_directory(ui, &path, app);
            }
        }
    });
}

// 名前変更のポップアップ
fn rename_popup(ui: &mut egui::Ui, ctx: &Context, app: &mut TxtEditorApp) {
    egui::Window::new("Rename").show(ctx, |ui| {
        ui.label("Enter new name (without extension):");
        ui.text_edit_singleline(&mut app.new_name);

        if ui.button("Rename").clicked() {
            if let Some(ref rename_target) = app.rename_target {
                if let Err(err) = rename_item(rename_target, &app.new_name) {
                    eprintln!("Failed to rename item: {}", err);
                } else {
                    if let Some(root_dir) = &app.folder_path {
                        app.file_list = get_txt_files_and_dirs_in_directory(root_dir.clone());
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

// 新しいフォルダのポップアップ
fn new_folder_popup(ui: &mut egui::Ui, ctx: &Context, app: &mut TxtEditorApp) {
    egui::Window::new("New Folder").show(ctx, |ui| {
        ui.label("Enter folder name:");
        ui.text_edit_singleline(&mut app.new_folder_name);

        if ui.button("Create").clicked() {
            if let Some(ref parent_dir) = app.new_folder_parent {
                if let Err(err) = create_folder(parent_dir, &app.new_folder_name) {
                    eprintln!("Failed to create folder: {}", err);
                } else {
                    if let Some(root_dir) = &app.folder_path {
                        app.file_list = get_txt_files_and_dirs_in_directory(root_dir.clone());
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
