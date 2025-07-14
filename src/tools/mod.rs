use eframe::egui::Ui;

mod json_util;
mod line_formatter;
mod timestamp;
mod url_util;

pub use json_util::JsonConverter;
pub use line_formatter::LineFormatter;
pub use timestamp::TimestampConverter;
pub use url_util::UrlConverter;

pub trait ToolItem {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn update(&mut self, ui: &mut Ui);
}
