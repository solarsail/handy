use eframe::egui::{FontFamily, FontId, TextFormat, text::LayoutJob};
use serde::{Deserialize, Deserializer};

use crate::style;

#[derive(Debug, PartialEq, Eq)]
pub(super) enum ServiceType {
    Issue,
    SimApi,
    SimMQ,
    SimCron,
    Other(String),
}

impl ServiceType {
    fn to_str(&self) -> &str {
        match self {
            Self::Issue => "issue",
            Self::SimApi => "sim-api",
            Self::SimMQ => "sim-mq",
            Self::SimCron => "sim-cron",
            Self::Other(s) => s.as_str(),
        }
    }

    fn service_names() -> Vec<&'static str> {
        vec![
            ServiceType::Issue.to_str(),
            ServiceType::SimApi.to_str(),
            ServiceType::SimMQ.to_str(),
            ServiceType::SimCron.to_str(),
        ]
    }
}

fn deserialize_service_type<'de, D>(deserializer: D) -> Result<ServiceType, D::Error>
where
    D: Deserializer<'de>,
{
    let buf = String::deserialize(deserializer)?;
    match buf.as_str() {
        "issue-mgmt" => Ok(ServiceType::Issue),
        "issue-issue-sim-api-api" => Ok(ServiceType::SimApi),
        "issue-issue-sim-mq-mq" => Ok(ServiceType::SimMQ),
        "issue-issue-sim-cron-cron" => Ok(ServiceType::SimCron),
        _ => Ok(ServiceType::Other(buf)),
    }
}

#[derive(Debug, Deserialize)]
pub(super) struct LogLine {
    pub(super) ts: String, // for WASM compatibility
    #[serde(rename = "rawLog")]
    pub(super) message: String,
    #[serde(rename = "serviceName", deserialize_with = "deserialize_service_type")]
    pub(super) service: ServiceType,
}

impl LogLine {
    pub(super) fn append_to_layout(&self, layout: &mut LayoutJob, dark_mode: bool) {
        let base_format = TextFormat {
            font_id: FontId {
                family: FontFamily::Monospace,
                ..Default::default()
            },
            ..Default::default()
        };
        layout.append(
            &self.ts,
            0.0,
            TextFormat {
                color: style::log_time_color(dark_mode),
                ..base_format.clone()
            },
        );
        layout.append(
            &self.service.to_str(),
            12.0,
            TextFormat {
                color: style::log_source_color(dark_mode),
                ..base_format.clone()
            },
        );
        layout.append(
            &self.message,
            12.0,
            TextFormat {
                color: style::log_message_color(dark_mode),
                ..base_format.clone()
            },
        );
    }
}

#[derive(Debug, Deserialize)]
pub(super) struct ClickhouseResponse {
    pub(super) data: Vec<LogLine>,
}
