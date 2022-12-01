use eframe::egui;

#[derive(PartialEq)]
pub enum FoldedOutLeft {
    None,
    Race,
    Host,
    Settings,
}

impl FoldedOutLeft {
    pub fn draw(&self, offset: [f32; 2], ctx: &egui::Context) {
        match self {
            Self::Race => {
                egui::Window::new("fold_out_left")
                    .anchor(egui::Align2::LEFT_TOP, offset)
                    .resizable(false)
                    .title_bar(false)
                    .show(ctx, |ui| {
                        ui.label("racing");
                    });
            },
            _ => {},
        }
    }
}
