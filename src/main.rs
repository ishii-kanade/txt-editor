mod app;
mod file_operations;
mod ui;

use eframe::NativeOptions;

fn main() {
    let app = app::TxtEditorApp::default();
    let native_options = NativeOptions::default();
    if let Err(e) = eframe::run_native(
        "Simple TXT Editor",
        native_options,
        Box::new(|_cc| Box::new(app)),
    ) {
        eprintln!("Error running the application: {}", e);
    }
}
