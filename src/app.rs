use eframe::egui::{Context, FontData, FontDefinitions, FontFamily, FontId, TextStyle};
use eframe::App;
use std::fs;
use std::path::PathBuf;

pub struct TxtEditorApp {
    pub folder_path: Option<PathBuf>,
    pub file_list: Vec<PathBuf>,
    pub selected_file: Option<PathBuf>,
    pub file_contents: String,
    pub font_size: f32,
    pub fonts_set: bool,
    pub file_modified: bool, // ファイルが編集されたかどうかを示すフラグ
}

impl Default for TxtEditorApp {
    fn default() -> Self {
        Self {
            folder_path: None,
            file_list: Vec::new(),
            selected_file: None,
            file_contents: String::new(),
            font_size: 16.0,
            fonts_set: false,
            file_modified: false,
        }
    }
}

impl TxtEditorApp {
    pub fn set_custom_fonts(&mut self, ctx: &Context) {
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

    fn save_file_if_modified(&mut self) {
        if self.file_modified {
            if let Some(ref selected_file) = self.selected_file {
                if let Err(err) = fs::write(selected_file, &self.file_contents) {
                    eprintln!("Failed to save file: {}", err);
                } else {
                    self.file_modified = false;
                }
            }
        }
    }
}

impl App for TxtEditorApp {
    fn update(&mut self, ctx: &Context, _: &mut eframe::Frame) {
        if !self.fonts_set {
            self.set_custom_fonts(ctx);
        }

        self.save_file_if_modified();

        crate::ui::display_top_panel(self, ctx);
        crate::ui::display_central_panel(self, ctx);
    }
}
