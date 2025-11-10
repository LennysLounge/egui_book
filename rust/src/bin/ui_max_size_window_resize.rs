use eframe::{
    App,
    egui::{self, Color32, ThemePreference, vec2},
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
        egui::Window::new("take_available_space")
            .default_size(vec2(150.0, 150.0))
            .show(ctx, |ui| {
                ui.painter()
                    .rect_filled(ui.max_rect(), 0.0, Color32::RED.linear_multiply(0.5));
                ui.label("I take available space");
                ui.take_available_space();
            });

        egui::Window::new("h centered")
            .default_size(vec2(150.0, 150.0))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.painter()
                        .rect_filled(ui.max_rect(), 0.0, Color32::RED.linear_multiply(0.5));
                    _ = ui.button("Im in a centered layout");
                });
            });

        egui::Window::new("v centered")
            .default_size(vec2(150.0, 150.0))
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    ui.painter()
                        .rect_filled(ui.max_rect(), 0.0, Color32::RED.linear_multiply(0.5));
                    _ = ui.button("Im in centered layout");
                });
            });
    }
}
