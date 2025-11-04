use eframe::{
    App,
    egui::{self, Direction, Id, Layout, ThemePreference},
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
    dir: Direction,
}
impl Default for MyApp {
    fn default() -> Self {
        Self {
            dir: Direction::TopDown,
        }
    }
}
impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top(Id::new("top")).show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Direction:");
                egui::ComboBox::new("id_salt", "")
                    .selected_text(format!("{:?}", self.dir))
                    .show_ui(ui, |ui| {
                        let variants = [
                            Direction::TopDown,
                            Direction::LeftToRight,
                            Direction::BottomUp,
                            Direction::RightToLeft,
                        ];
                        for dir in variants {
                            if ui
                                .selectable_label(self.dir == dir, format!("{:?}", dir))
                                .clicked()
                            {
                                self.dir = dir
                            };
                        }
                    });
            })
        });
        // ANCHOR: sample
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(
                Layout {
                    main_dir: self.dir,
                    main_wrap: false,
                    main_align: egui::Align::Min,
                    main_justify: false,
                    cross_align: egui::Align::Min,
                    cross_justify: false,
                },
                |ui| {
                    ui.label("label 1");
                    _ = ui.button("button 2");
                    let mut checked = true;
                    ui.checkbox(&mut checked, "checkbox 3");
                    ui.label("label 4");
                    _ = ui.button("button 5");
                    let mut checked = true;
                    ui.checkbox(&mut checked, "checkbox 6");
                },
            );
        });
        // ANCHOR_END: sample
    }
}
