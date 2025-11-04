use eframe::{
    App,
    egui::{self, Color32, Id, ThemePreference},
};

#[path = "../common.rs"]
mod common;

fn main() {
    common::run_app(&|cc| {
        cc.egui_ctx.set_theme(ThemePreference::Dark);
        MyApp::default()
    });
}

struct MyApp {
    count: i32,
}
impl Default for MyApp {
    fn default() -> Self {
        Self { count: 1 }
    }
}
impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top(Id::new("top")).show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("add").clicked() {
                    self.count += 1;
                }
                if ui.button("remove").clicked() {
                    if self.count > 0 {
                        self.count -= 1;
                    }
                }
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            for i in 0..self.count {
                ui.label(format!("label {i}"));
            }
            let cursor = ui.cursor();
            ui.painter()
                .rect_filled(cursor, 0.0, Color32::RED.linear_multiply(0.7));
        });
    }
}
