use eframe::egui;
use chrono::prelude::*;

use crate::backend::Event;

pub enum FoldedOutLeft {
    None,
    Race(Box<Vec<Event>>),
    Host,
    Settings,
}

impl PartialEq for FoldedOutLeft {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

impl FoldedOutLeft {
    pub fn draw(&self, offset: [f32; 2], draw_size: [f32; 2], ctx: &egui::Context) {
        match self {
            Self::Race(events) => {
                egui::Window::new("fold_out_left")
                    .anchor(egui::Align2::LEFT_TOP, offset)
                    .resizable(false)
                    .title_bar(false)
                    .fixed_size(draw_size)
                    .show(ctx, |ui| {
                        ui.heading("Events");
                        for event in events.iter() {
                            ui.columns(4, |columns| {
                                columns[0].label(format!("{:?}", event.series));
                                let time = Utc.timestamp_opt(event.timestamp, 0).unwrap();
                                columns[1].label(time.format("%Y-%m-%d %H:%M:%S.%f").to_string());
                                columns[2].label(&event.id);
                                columns[3].button("Join");
                            });
                        }
                    });
            }
            Self::Host => {
                egui::Window::new("fold_out_left")
                    .anchor(egui::Align2::LEFT_TOP, offset)
                    .resizable(false)
                    .title_bar(false)
                    .show(ctx, |ui| {
                        ui.label("host racing");
                    });
            }
            Self::Settings => {
                egui::Window::new("fold_out_left")
                    .anchor(egui::Align2::LEFT_TOP, offset)
                    .resizable(false)
                    .title_bar(false)
                    .show(ctx, |ui| {
                        ui.label("settings");
                    });
            }
            _ => {},
        }
    }
}
