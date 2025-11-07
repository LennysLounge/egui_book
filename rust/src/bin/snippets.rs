use eframe::{
    App,
    egui::{self, Sense, ThemePreference, vec2},
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
            #[rustfmt::skip]
            ui.scope(|ui|{
// ANCHOR: allocate_space
let (id, rect) = ui.allocate_space(vec2(66.0, 33.0));
// ANCHOR_END: allocate_space
let (_id, _rect) = (id, rect);
            });

            #[rustfmt::skip]
            ui.scope(|ui|{
// ANCHOR: allocate_and_interact
let (id, rect) = ui.allocate_space(vec2(66.0, 33.0));
let response = ui.interact(rect, id, Sense::click());
// ANCHOR_END: allocate_and_interact
let _ = response;
            });

            #[rustfmt::skip]
            ui.scope(|ui|{
// ANCHOR: allocate_response
let response = ui.allocate_response(vec2(66.0, 33.0), Sense::click());
// ANCHOR_END: allocate_response
let _ = response;
            });
        });
    }
}
