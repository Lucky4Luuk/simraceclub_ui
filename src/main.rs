#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

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
}

impl Default for App {
    fn default() -> Self {
        Self {
            bg0: None,
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

        egui::Window::new("window")
            .anchor(egui::Align2::CENTER_TOP, [0.0, draw_size.y / 36.0])
            .resizable(false)
            .fixed_size([draw_size.x / 2.0, draw_size.y / 6.0])
            .title_bar(false)
            .show(ctx, |ui| {
                ui.style_mut().visuals.widgets.noninteractive.bg_fill = egui::Color32::TRANSPARENT;
                ui.columns(3, |columns| {
                    columns[0].centered_and_justified(|ui| {
                        if ui.button("Left").clicked() {
                            println!("Left!");
                        }
                    });
                    columns[1].centered_and_justified(|ui| {
                        if ui.button("Play").clicked() {
                            println!("Pressed!");
                        }
                    });
                    columns[2].centered_and_justified(|ui| {
                        if ui.button("Right").clicked() {
                            println!("Right!");
                        }
                    });
                })
            });
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
