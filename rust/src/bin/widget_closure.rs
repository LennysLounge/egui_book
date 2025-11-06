use eframe::{
    App,
    egui::{self, ThemePreference, Ui},
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
            {
                #[rustfmt::skip]
                // ANCHOR: widget_closure
ui.add(|ui: &mut Ui| {
    _ = ui.button("im a button");
    ui.label("hello world")
});
                // ANCHOR_END: widget_closure

                #[rustfmt::skip]
                ui.scope(|ui|{
// ANCHOR: combo_box
egui::ComboBox::new("combo_box", "This is a ComboBox")
    .selected_text("None yet")
    .show_ui(ui, |ui| {
        _ = ui.selectable_label(false, "one");
        _ = ui.selectable_label(false, "two");
        _ = ui.selectable_label(false, "three");
    });
// ANCHOR_END: combo_box
                });
            }
        });
    }
}
