use eframe::{egui, epi};

pub struct TemplateApp {
    label: String,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            label: "".to_owned(),
        }
    }
}

impl epi::App for TemplateApp {
    fn name(&self) -> &str {
        "cv-visualiser-gui"
    }


    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        let Self { label } = self;

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
