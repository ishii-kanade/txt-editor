use std::fs;
use std::path::PathBuf;

use eframe::egui::{
    self, CentralPanel, Context, FontData, FontDefinitions, FontFamily, FontId, TextEdit,
    TextStyle, TopBottomPanel,
};
use eframe::{App, Frame, NativeOptions};

#[derive(Default)]
struct TxtEditorApp {
    folder_path: Option<PathBuf>,
    file_list: Vec<PathBuf>,
    selected_file: Option<PathBuf>,
    file_contents: String,
    font_size: f32,
    fonts_set: bool, // フォントが設定されているかを追跡するフラグ
}

impl TxtEditorApp {
    fn set_custom_fonts(&mut self, ctx: &Context) {
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "custom_font".to_owned(),
            FontData::from_static(include_bytes!("NotoSansJP-Regular.ttf")),
        );
        fonts
            .families
            .entry(FontFamily::Proportional)
            .or_default()
            .push("custom_font".to_owned());
        fonts
            .families
            .entry(FontFamily::Monospace)
            .or_default()
            .push("custom_font".to_owned());
        ctx.set_fonts(fonts);

        let mut style = (*ctx.style()).clone();
        let default_font_size = self.font_size.max(16.0);
        style.text_styles = [
            (
                TextStyle::Heading,
                FontId::new(default_font_size + 6.0, FontFamily::Proportional),
            ),
            (
                TextStyle::Body,
                FontId::new(default_font_size, FontFamily::Proportional),
            ),
            (
                TextStyle::Monospace,
                FontId::new(default_font_size, FontFamily::Monospace),
            ),
            (
                TextStyle::Button,
                FontId::new(default_font_size, FontFamily::Proportional),
            ),
            (
                TextStyle::Small,
                FontId::new(default_font_size - 2.0, FontFamily::Proportional),
            ),
        ]
        .into();
        ctx.set_style(style);
        self.fonts_set = true;
    }

    fn display_top_panel(&mut self, ctx: &Context) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Select Folder").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        self.folder_path = Some(path.clone());
                        self.file_list = fs::read_dir(path)
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

    fn display_central_panel(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            if let Some(ref folder_path) = self.folder_path {
                ui.label(format!("Directory: {}", folder_path.display()));
                ui.separator();

                for file in &self.file_list {
                    if ui
                        .selectable_label(false, file.file_name().unwrap().to_string_lossy())
                        .clicked()
                    {
                        self.selected_file = Some(file.clone());
                        self.file_contents = fs::read_to_string(file)
                            .unwrap_or_else(|_| "Failed to read file".to_string());
                    }
                }
            }

            if let Some(ref selected_file) = self.selected_file {
                ui.separator();
                ui.label(format!("Selected File: {}", selected_file.display()));
                ui.add(
                    TextEdit::multiline(&mut self.file_contents)
                        .font(egui::TextStyle::Monospace)
                        .desired_rows(10),
                );
            }
        });
    }
}

impl App for TxtEditorApp {
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        if !self.fonts_set {
            self.set_custom_fonts(ctx);
        }

        self.display_top_panel(ctx);
        self.display_central_panel(ctx);
    }
}

fn main() {
    let app = TxtEditorApp::default();
    let native_options = NativeOptions::default();
    eframe::run_native(
        "Simple TXT Editor",
        native_options,
        Box::new(|_cc| Box::new(app)),
    );
}
