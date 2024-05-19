use crate::app::TxtEditorApp;
use eframe::egui::{self, CentralPanel, Context, ScrollArea, TextEdit};

pub fn display(app: &mut TxtEditorApp, ctx: &Context) {
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
