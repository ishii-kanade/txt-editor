use crate::app::TxtEditorApp;
use eframe::egui::{self, Context, ScrollArea, SidePanel, TextEdit};

pub fn display(app: &mut TxtEditorApp, ctx: &Context) {
    SidePanel::right("right_panel").show(ctx, |ui| {
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
