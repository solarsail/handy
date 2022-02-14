use eframe::egui::Ui;

mod timestamp;

pub use timestamp::TimestampConverter;

pub trait ToolItem {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn update(&mut self, ui: &mut Ui);
}
