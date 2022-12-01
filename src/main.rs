#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

mod ui_left;
use ui_left::FoldedOutLeft;

fn main() {
    let mut options = eframe::NativeOptions::default();
    options.maximized = true;
    eframe::run_native(
        "SimRace.club",
        options,
        Box::new(|_cc| Box::new(App::default())),
    );
}

struct App {
    bg0: Option<egui::TextureHandle>,

    folded_out_left: FoldedOutLeft,
}

impl Default for App {
    fn default() -> Self {
        Self {
            bg0: None,

            folded_out_left: FoldedOutLeft::None,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().frame(egui::containers::Frame::none()).show(ctx, |ui| {
            let texture: &egui::TextureHandle = self.bg0.get_or_insert_with(|| {
                // Load texture from disk
                let image = image_from_path("bg0.png");

                // Load texture into egui
                ui.ctx().load_texture(
                    "bg0",
                    image.unwrap_or(egui::ColorImage::example()),
                    egui::TextureFilter::Linear
                )
            });
            let texture_size = texture.size_vec2();
            let aspect_ratio = texture_size.x / texture_size.y;
            let mut draw_size = ui.clip_rect().max - ui.clip_rect().min;
            let draw_aspect_ratio = draw_size.x / draw_size.y;
            if draw_aspect_ratio > aspect_ratio {
                draw_size.y /= aspect_ratio / draw_aspect_ratio;
            } else {
                draw_size.x *= aspect_ratio / draw_aspect_ratio;
            }
            ui.image(texture, draw_size);
        });
        let draw_size = frame.info().window_info.size;

        let offset = draw_size.y / 56.0;

        egui::Window::new("main_controls")
            .anchor(egui::Align2::LEFT_TOP, [offset, offset])
            .resizable(false)
            .fixed_size([draw_size.x / 5.0, draw_size.y / 32.0])
            .title_bar(false)
            .show(ctx, |ui| {
                ui.style_mut().visuals.widgets.noninteractive.bg_fill = egui::Color32::TRANSPARENT;
                ui.style_mut().visuals.widgets.inactive.bg_fill = egui::Color32::TRANSPARENT;
                ui.style_mut().visuals.widgets.hovered.bg_fill = egui::Color32::TRANSPARENT;
                ui.style_mut().visuals.widgets.active.bg_fill = egui::Color32::TRANSPARENT;
                ui.style_mut().visuals.widgets.open.bg_fill = egui::Color32::TRANSPARENT;
                ui.columns(3, |columns| {
                    columns[0].centered_and_justified(|ui| {
                        if ui.button("Race").clicked() {
                            if self.folded_out_left != FoldedOutLeft::Race {
                                self.folded_out_left = FoldedOutLeft::Race;
                            } else {
                                self.folded_out_left = FoldedOutLeft::None;
                            }
                        }
                    });
                    columns[1].centered_and_justified(|ui| {
                        if ui.button("Host").clicked() {
                            println!("Host!");
                        }
                    });
                    columns[2].centered_and_justified(|ui| {
                        if ui.button("Settings").clicked() {
                            println!("Settings!");
                        }
                    });
                });
            });

        egui::Window::new("profile_controls")
            .anchor(egui::Align2::RIGHT_TOP, [-offset, offset])
            .resizable(false)
            .fixed_size([draw_size.x / 3.0, draw_size.y / 32.0])
            .title_bar(false)
            .show(ctx, |ui| {
                ui.style_mut().visuals.widgets.noninteractive.bg_fill = egui::Color32::TRANSPARENT;
                ui.style_mut().visuals.widgets.inactive.bg_fill = egui::Color32::TRANSPARENT;
                ui.style_mut().visuals.widgets.hovered.bg_fill = egui::Color32::TRANSPARENT;
                ui.style_mut().visuals.widgets.active.bg_fill = egui::Color32::TRANSPARENT;
                ui.style_mut().visuals.widgets.open.bg_fill = egui::Color32::TRANSPARENT;
                ui.columns(5, |columns| {
                    columns[0].centered_and_justified(|ui| {
                        ui.label("Pace Rating: 1200");
                    });
                    columns[1].centered_and_justified(|ui| {
                        ui.label("Safety Rating: 1600");
                    });
                    columns[2].centered_and_justified(|ui| {
                        if ui.button("Profile").clicked() {
                            println!("Profile!");
                        }
                    });
                    columns[3].centered_and_justified(|ui| {
                        if ui.button("Garage").clicked() {
                            println!("Garage!");
                        }
                    });
                    columns[4].centered_and_justified(|ui| {
                        if ui.button("Stats").clicked() {
                            println!("Settings!");
                        }
                    });
                });
            });

        self.folded_out_left.draw([offset, offset*2.5 + draw_size.y / 32.0], ctx);
    }
}

fn image_from_path(path: &str) -> Result<egui::ColorImage, image::ImageError> {
    let image = image::io::Reader::open(path)?.decode()?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}
