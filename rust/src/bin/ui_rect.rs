use eframe::{
    App,
    egui::{self, ThemePreference},
};

#[path = "../common.rs"]
mod common;

fn main() {
    common::run_app(&|cc| {
        cc.egui_ctx.set_theme(ThemePreference::Dark);
        MyApp
    });
}

struct MyApp;
impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::Window::new("window").show(ctx, |ui| {
            ui.label(format!("{:?}", ui.max_rect()));
            ui.painter()
                .rect_filled(ui.max_rect(), 0.0, egui::Color32::RED.linear_multiply(0.5));
        });
        // egui::CentralPanel::default().show(ctx, |ui| {
        //     ui.painter()
        //         .rect_filled(ui.max_rect(), 0.0, egui::Color32::RED.linear_multiply(0.5));
        // });
    }
}
