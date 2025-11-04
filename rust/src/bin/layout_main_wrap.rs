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
    should_wrap: bool,
    count: i32,
}
impl Default for MyApp {
    fn default() -> Self {
        Self {
            dir: Direction::LeftToRight,
            should_wrap: true,
            count: 15,
        }
    }
}
impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top(Id::new("top")).show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("add label").clicked() {
                    self.count += 1;
                }
                if ui.button("remove label").clicked() {
                    if self.count > 0 {
                        self.count -= 1;
                    }
                }
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
                ui.label("should wrap:");
                ui.checkbox(&mut self.should_wrap, "");
            })
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(
                Layout {
                    main_dir: self.dir,
                    main_wrap: self.should_wrap,
                    main_align: egui::Align::Min,
                    main_justify: false,
                    cross_align: egui::Align::Min,
                    cross_justify: false,
                },
                |ui| {
                    for i in 0..self.count {
                        ui.label(format!("label {i}"));
                    }
                },
            );
        });
    }
}
