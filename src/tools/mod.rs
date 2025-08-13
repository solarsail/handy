use eframe::egui::Ui;

mod issue_jump;
mod json_util;
mod line_formatter;
mod log_ck;
mod taskgraph;
mod timestamp;
mod url_util;

pub use issue_jump::IssueJump;
pub use json_util::JsonConverter;
pub use line_formatter::LineFormatter;
pub use log_ck::LogRetriever;
pub use taskgraph::TaskGraphJump;
pub use timestamp::TimestampConverter;
pub use url_util::UrlConverter;

pub trait ToolItem {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn update(&mut self, ui: &mut Ui);
}
