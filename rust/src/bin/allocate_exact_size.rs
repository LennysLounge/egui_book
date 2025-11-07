use eframe::{
    App,
    egui::{self, Align, Color32, Layout, Sense, ThemePreference, vec2},
};

#[path = "../common.rs"]
mod common;

fn main() {
    common::run_app(&|cc| {
        cc.egui_ctx.set_theme(ThemePreference::Dark);
        MyApp{
            checked: false,
            align: Align::Center,
        }
    });
}

struct MyApp {
    checked: bool,
    align: Align,
}
impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("asd").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Align:");
                egui::ComboBox::new("id_salt", "")
                    .selected_text(format!("{:?}", self.align))
                    .show_ui(ui, |ui| {
                        let variants = [Align::Min, Align::Center, Align::Max];
                        for dir in variants {
                            if ui
                                .selectable_label(self.align == dir, format!("{:?}", dir))
                                .clicked()
                            {
                                self.align = dir
                            };
                        }
                    });
            })
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            // The layout is justified which means that widget might be bigger than requested
            ui.with_layout(Layout::top_down_justified(self.align), |ui| {
                // a button will expand its frame to fill the entire width
                _ = ui.button("I will fill the entire width");

                // ANCHOR: allocate_exact_size
                let (rect, res) = ui.allocate_exact_size(vec2(66.0, 33.0), Sense::click());
                ui.painter().rect_stroke(
                    res.rect,
                    0.0,
                    (2.0, Color32::RED),
                    egui::StrokeKind::Inside,
                );
                ui.painter().rect_stroke(
                    rect,
                    0.0,
                    (2.0, Color32::LIGHT_BLUE),
                    egui::StrokeKind::Inside,
                );
                // ANCHOR_END: allocate_exact_size

                // a checkbox should not be rendered at full width
                ui.checkbox(&mut self.checked, "");
            });
        });
    }
}
