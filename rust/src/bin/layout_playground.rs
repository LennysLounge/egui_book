use eframe::{
    App,
    egui::{
        self, Align, Color32, Direction, Frame, Grid, Id, Layout, Margin, Slider, ThemePreference,
    },
};

#[path = "../common.rs"]
mod common;

fn main() {
    common::run_app(&|cc| {
        cc.egui_ctx.set_theme(ThemePreference::Dark);
        MyApp::default()
    });
}

enum Widget {
    Button,
    Label,
    CheckBox,
}

struct MyApp {
    layout: Layout,
    widgets: Vec<Widget>,
    margin: i8,
    draw_rects: bool,
}
impl Default for MyApp {
    fn default() -> Self {
        Self {
            layout: Layout::top_down(Align::Min),
            widgets: vec![
                Widget::Label,
                Widget::Button,
                Widget::CheckBox,
                Widget::Label,
                Widget::Button,
                Widget::CheckBox,
            ],
            margin: 8,
            draw_rects: false,
        }
    }
}
impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left(Id::new("side panel")).show(ctx, |ui| {
            Grid::new(Id::new("grid")).show(ui, |ui| {
                ui.label("main direction");
                egui::ComboBox::new("main_dir", "")
                    .selected_text(format!("{:?}", self.layout.main_dir))
                    .show_ui(ui, |ui| {
                        let variants = [
                            Direction::TopDown,
                            Direction::LeftToRight,
                            Direction::BottomUp,
                            Direction::RightToLeft,
                        ];
                        for dir in variants {
                            if ui
                                .selectable_label(self.layout.main_dir == dir, format!("{:?}", dir))
                                .clicked()
                            {
                                self.layout.main_dir = dir
                            };
                        }
                    });
                ui.end_row();

                ui.label("main align");
                egui::ComboBox::new("main_align", "")
                    .selected_text(format!("{:?}", self.layout.main_align))
                    .show_ui(ui, |ui| {
                        let variants = [Align::Min, Align::Center, Align::Max];
                        for dir in variants {
                            if ui
                                .selectable_label(
                                    self.layout.main_align == dir,
                                    format!("{:?}", dir),
                                )
                                .clicked()
                            {
                                self.layout.main_align = dir
                            };
                        }
                    });
                ui.end_row();

                ui.label("main justify");
                ui.checkbox(&mut self.layout.main_justify, "");
                ui.end_row();

                ui.label("cross align");
                egui::ComboBox::new("cross_align", "")
                    .selected_text(format!("{:?}", self.layout.cross_align))
                    .show_ui(ui, |ui| {
                        let variants = [Align::Min, Align::Center, Align::Max];
                        for dir in variants {
                            if ui
                                .selectable_label(
                                    self.layout.cross_align == dir,
                                    format!("{:?}", dir),
                                )
                                .clicked()
                            {
                                self.layout.cross_align = dir
                            };
                        }
                    });
                ui.end_row();

                ui.label("cross justify");
                ui.checkbox(&mut self.layout.cross_justify, "");
                ui.end_row();
            });


            ui.separator();
            if ui.button("add button").clicked() {
                self.widgets.push(Widget::Button);
            }
            if ui.button("add label").clicked() {
                self.widgets.push(Widget::Label);
            }
            if ui.button("add checkbox").clicked() {
                self.widgets.push(Widget::CheckBox);
            }
            if ui.button("remove last").clicked() {
                self.widgets.pop();
            }

            ui.separator();
            ui.checkbox(&mut self.draw_rects, "Draw sizes");

            ui.separator();
            ui.label("This is not part of a layout but might be interesting to play around with");
            ui.label("Inner margin:");
            ui.add(Slider::new(&mut self.margin, 0..=50));
            ui.take_available_width();
        });
        egui::CentralPanel::default()
            .frame(Frame {
                inner_margin: Margin::same(self.margin),
                ..Frame::central_panel(&ctx.style())
            })
            .show(ctx, |ui| {
                ui.with_layout(self.layout, |ui| {
                    for (i, widget) in self.widgets.iter().enumerate() {
                        let res = match widget {
                            Widget::Button => ui.button(format!("button {i}")),
                            Widget::Label => ui.label(format!("label {i}")),
                            Widget::CheckBox => {
                                let mut b = true;
                                ui.checkbox(&mut b, format!("checkbox {i}"))
                            }
                        };
                        if self.draw_rects {
                            ui.painter().rect_stroke(
                                res.rect,
                                0.0,
                                (1.0, Color32::RED),
                                egui::StrokeKind::Inside,
                            );
                        }
                    }
                });
            });
    }
}
