use crate::app::TxtEditorApp;
use eframe::egui::{self, CentralPanel, Color32, Context, ScrollArea, TextEdit, TopBottomPanel};
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

            // 文字数をカウントして表示
            let char_count = app.file_contents.chars().count();
            ui.label(format!("Character count: {}", char_count));
        });
    });
}

pub fn display_side_panel(app: &mut TxtEditorApp, ctx: &Context) {
    eframe::egui::SidePanel::left("side_panel").show(ctx, |ui| {
        if let Some(ref folder_path) = app.folder_path {
            ui.label(format!("Directory: {}", folder_path.display()));
            ui.separator();

            for file in &app.file_list {
                let file_name = file.file_name().unwrap().to_string_lossy();
                let is_selected = Some(file) == app.selected_file.as_ref();

                let label = if is_selected {
                    ui.colored_label(Color32::YELLOW, file_name)
                } else {
                    ui.label(file_name)
                };

                if label.clicked() {
                    app.selected_file = Some(file.clone());
                    app.file_contents = fs::read_to_string(file)
                        .unwrap_or_else(|_| "Failed to read file".to_string());
                    app.file_modified = false; // ファイルを選択したときは未編集とする
                }
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
