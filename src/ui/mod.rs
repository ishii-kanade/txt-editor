pub mod central_panel;
pub mod right_panel;
pub mod side_panel;
pub mod top_panel;

use crate::app::TxtEditorApp;
use eframe::egui::Context;

pub fn display_top_panel(app: &mut TxtEditorApp, ctx: &Context) {
    top_panel::display(app, ctx);
}

pub fn display_side_panel(app: &mut TxtEditorApp, ctx: &Context) {
    side_panel::display(app, ctx);
}

pub fn display_central_panel(app: &mut TxtEditorApp, ctx: &Context) {
    central_panel::display(app, ctx);
}

pub fn display_right_panel(app: &mut TxtEditorApp, ctx: &Context) {
    right_panel::display(app, ctx);
}
