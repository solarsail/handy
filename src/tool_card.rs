use eframe::egui::{widgets::Label, Color32, RichText, Ui};

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
        let color = if active {
            Color32::WHITE
        } else {
            Color32::GRAY
        };
        ui.add(Label::new(
            RichText::new(self.caption).heading().color(color),
        ));
        ui.add(Label::new(RichText::new(self.description).color(color)));
    }
}
