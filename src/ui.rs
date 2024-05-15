use crate::app::TxtEditorApp;
use eframe::egui::{self, CentralPanel, Context, TextEdit, TopBottomPanel};
use std::fs;

pub fn display_top_panel(app: &mut TxtEditorApp, ctx: &Context) {
    TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            if ui.button("Select Folder").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    app.folder_path = Some(path.clone());
                    app.file_list = fs::read_dir(path)
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
                        .collect();
                }
            }
        });
    });
}

pub fn display_central_panel(app: &mut TxtEditorApp, ctx: &Context) {
    CentralPanel::default().show(ctx, |ui| {
        if let Some(ref folder_path) = app.folder_path {
            ui.label(format!("Directory: {}", folder_path.display()));
            ui.separator();

            for file in &app.file_list {
                if ui
                    .selectable_label(false, file.file_name().unwrap().to_string_lossy())
                    .clicked()
                {
                    app.selected_file = Some(file.clone());
                    app.file_contents = fs::read_to_string(file)
                        .unwrap_or_else(|_| "Failed to read file".to_string());
                    app.file_modified = false; // ファイルを選択したときは未編集とする
                }
            }
        }

        if let Some(ref selected_file) = app.selected_file {
            ui.separator();
            ui.label(format!("Selected File: {}", selected_file.display()));
            let response = ui.add(
                TextEdit::multiline(&mut app.file_contents)
                    .font(egui::TextStyle::Monospace)
                    .desired_rows(10),
            );

            if response.changed() {
                app.file_modified = true; // テキストが変更されたときにフラグを設定する
            }
        }
    });
}
