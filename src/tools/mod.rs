use eframe::egui::Ui;

mod json_util;
mod timestamp;

pub use json_util::JsonConverter;
pub use timestamp::TimestampConverter;

pub trait ToolItem {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn update(&mut self, ui: &mut Ui);
}
