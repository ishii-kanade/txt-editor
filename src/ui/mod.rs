pub mod central_panel;
pub mod left_panel;
pub mod right_panel;
pub mod top_panel;
pub mod utils;

use crate::app::TxtEditorApp;
use eframe::egui::Context;

pub fn display_top_panel(app: &mut TxtEditorApp, ctx: &Context) {
    top_panel::display(app, ctx);
}

pub fn display_left_panel(app: &mut TxtEditorApp, ctx: &Context) {
    left_panel::display(app, ctx);
}

pub fn display_central_panel(app: &mut TxtEditorApp, ctx: &Context) {
    central_panel::display(app, ctx);
}

pub fn display_right_panel(app: &mut TxtEditorApp, ctx: &Context) {
    right_panel::display(app, ctx);
}
