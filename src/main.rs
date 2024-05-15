mod app;
mod ui;

use eframe::NativeOptions;

fn main() {
    let app = app::TxtEditorApp::default();
    let native_options = NativeOptions::default();
    eframe::run_native(
        "Simple TXT Editor",
        native_options,
        Box::new(|_cc| Box::new(app)),
    );
}
