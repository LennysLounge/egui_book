use eframe::{
    App,
    egui::{self, Id, ThemePreference, pos2},
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
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            if ui.button("reset").clicked() {
                ui.data_mut(|d| d.clear());
            }
        });
        egui::Window::new("Infinit growth")
            .id(Id::new("window"))
            .default_height(0.0)
            .default_pos(pos2(100.0, 50.0))
            .show(ctx, |ui| {
                ui.label("My hight will always grow");

                ui.add_space(ui.available_height() + 1.0);
                ui.ctx().request_repaint();
            });
    }
}
