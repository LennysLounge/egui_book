use eframe::{
    App,
    egui::{self, Shape, ThemePreference},
};

#[path = "../common.rs"]
mod common;

fn main() {
    common::run_app(&|cc| {
        cc.egui_ctx.set_theme(ThemePreference::Dark);
        MyApp { count: 3 }
    });
}

struct MyApp {
    count: i32,
}
impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("add").clicked() {
                    self.count += 1;
                }
                if ui.button("remove").clicked() && self.count > 0 {
                    self.count -= 1;
                }
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.scope(|ui| {
                let rect = ui.painter().add(Shape::Noop);
                for i in 0..self.count {
                    let mut s = String::new();
                    for j in 0..i {
                        s.push_str(&format!("{j}"));
                    }
                    ui.label(format!("label {s}"));
                }
                ui.painter().set(
                    rect,
                    Shape::rect_filled(
                        ui.min_rect(),
                        0.0,
                        egui::Color32::BLUE.linear_multiply(0.5),
                    ),
                );
            });
        });
    }
}
