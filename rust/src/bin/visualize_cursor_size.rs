use eframe::{
    App,
    egui::{self, Color32, ThemePreference},
};

#[path = "../common.rs"]
mod common;

fn main() {
    common::run_app(&|cc| {
        cc.egui_ctx.set_theme(ThemePreference::Dark);
        MyApp::default()
    });
}

struct MyApp;
impl Default for MyApp {
    fn default() -> Self {
        Self
    }
}
impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(format!("Cursor debug print: {:?}", ui.cursor()));
            ui.label(format!("Cursor height: {}", ui.cursor().height()));
            ui.painter()
                .rect_filled(ui.cursor(), 0.0, Color32::RED.linear_multiply(0.7));
        });
    }
}
