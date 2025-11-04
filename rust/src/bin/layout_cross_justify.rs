use eframe::{
    App,
    egui::{self, Align, Direction, Id, Layout, Rect, ThemePreference, Ui},
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
    align: Align,
    cross_justify: bool,
    draw_rects: bool,
}
impl Default for MyApp {
    fn default() -> Self {
        Self {
            dir: Direction::TopDown,
            align: Align::Min,
            cross_justify: true,
            draw_rects: false,
        }
    }
}
impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top(Id::new("top")).show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Direction:");
                egui::ComboBox::new("direction", "")
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
                ui.label("cross align:");
                egui::ComboBox::new("align", "")
                    .selected_text(format!("{:?}", self.align))
                    .show_ui(ui, |ui| {
                        let variants = [Align::Max, Align::Center, Align::Min];
                        for v in variants {
                            if ui
                                .selectable_label(self.align == v, format!("{:?}", v))
                                .clicked()
                            {
                                self.align = v;
                            };
                        }
                    });
                ui.label("cross justify:");
                ui.checkbox(&mut self.cross_justify, "");
            });
            ui.horizontal(|ui|{
                ui.label("Draw response rectangles:");
                ui.checkbox(&mut self.draw_rects, "");
            })
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(
                Layout {
                    main_dir: self.dir,
                    main_wrap: false,
                    main_align: Align::Min,
                    main_justify: false,
                    cross_align: self.align,
                    cross_justify: self.cross_justify,
                },
                |ui| {
                    let r = ui.label("label 1");
                    if self.draw_rects{
                        draw_rect(ui, r.rect);
                    }
                    let r = ui.button("button 2");
                    if self.draw_rects{
                        draw_rect(ui, r.rect);
                    }
                    let mut checked = true;
                    let r = ui.checkbox(&mut checked, "checkbox 3");
                    if self.draw_rects{
                        draw_rect(ui, r.rect);
                    }
                    let r = ui.label("label 4");
                    if self.draw_rects{
                        draw_rect(ui, r.rect);
                    }
                    let r = ui.button("button 5");
                    if self.draw_rects{
                        draw_rect(ui, r.rect);
                    }
                    let mut checked = true;
                    let r = ui.checkbox(&mut checked, "checkbox 6");
                    if self.draw_rects{
                        draw_rect(ui, r.rect);
                    }
                },
            );
        });
    }
}

fn draw_rect(ui: &mut Ui, rect: Rect){
    ui.painter().rect_stroke(rect, 0.0, (1.0, egui::Color32::RED), egui::StrokeKind::Inside);
}
