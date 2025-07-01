use eframe::egui::{Color32, RichText, Ui, widgets::Label};

pub struct ToolCard<'a> {
    caption: &'a str,
    description: &'a str,
}

impl<'a> ToolCard<'a> {
    pub fn new(caption: &'a str, description: &'a str) -> Self {
        ToolCard {
            caption,
            description,
        }
    }

    pub fn update(&mut self, ui: &mut Ui, active: bool) {
        let color = match (active, ui.visuals().dark_mode) {
            (true, true) => Color32::WHITE,  // dark & active
            (false, true) => Color32::GRAY,  // dark & inactive
            (true, false) => Color32::BLACK, // light & active
            (false, false) => Color32::GRAY, // light & inactive
        };
        ui.vertical(|ui| {
            ui.set_min_height(60.0);
            ui.add(Label::new(
                RichText::new(self.caption).heading().color(color),
            ));
            ui.add(Label::new(RichText::new(self.description).color(color)));
        });
    }
}
