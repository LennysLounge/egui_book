use eframe::{
    App,
    egui::{self, Color32, ThemePreference},
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
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("This is the parent ui");
            _ = ui.button("Normal button");
            ui.separator();
            ui.scope(|ui| {
                ui.label("Now we are in a scope and disable the button frame");
                ui.style_mut().visuals.button_frame = false;
                _ = ui.button("Button without frame");
            });
            ui.separator();
            ui.scope(|ui| {
                ui.label("Now we are in a second scope and set the text color blue");
                ui.style_mut().visuals.override_text_color = Some(Color32::BLUE);
                _ = ui.button("Blue text button");
            });
            ui.separator();
            ui.label("Back in the parent scope and everything is normal");
            _ = ui.button("Normal button again");
        });
    }
}
