use eframe::{egui, epi};

pub struct CvGui {
    label: String,
    images: Vec<egui::TextureId>,
}

pub struct ImageView {}

fn display_image(ui: &mut egui::Ui, frame: &mut epi::Frame<'_>, image: &ImageView) {}

impl Default for CvGui {
    fn default() -> Self {
        Self {
            label: "".to_owned(),
            images: vec![],
        }
    }
}

impl CvGui {
    fn store_image(&mut self, frame: &mut epi::Frame<'_>, image: Vec<u8>) {
        // self.images.push(frame.tex_allocator().alloc_srgba_premultiplied(..));
    }
}

impl epi::App for CvGui {
    fn name(&self) -> &str {
        "cv-visualiser-gui"
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        let Self { label, images } = self;

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Search");
            ui.horizontal(|ui| {
                ui.label("Name: ");
                ui.text_edit_singleline(label);
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add(
                    egui::Hyperlink::new("https://github.com/emilk/egui/").text("powered by egui"),
                );
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Pipeline");
            egui::warn_if_debug_build(ui);
        });
    }
}
