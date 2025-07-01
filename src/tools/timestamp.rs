use chrono::{LocalResult, prelude::*};
use eframe::egui::{Button, Color32, RichText, Ui, widgets::Label};

const DT_FORMAT_S: &str = "%F %T";
const DT_FORMAT_MS: &str = "%F %T%.3f";
const DT_FORMAT_US: &str = "%F %T%.6f";
const DT_FORMAT_NS: &str = "%F %T%.9f";

#[derive(PartialEq, Eq)]
enum InputType {
    Invalid,
    Timestamp,
    DateTimeStr,
}

#[derive(PartialEq, Eq)]
enum TimeUnit {
    Sec,
    Milli,
    Micro,
    Nano,
}

pub struct TimestampConverter {
    input: String,
    converted: String,
    format_warning: &'static str,
    copied_prompt: &'static str,
    unit: TimeUnit,
}

impl super::ToolItem for TimestampConverter {
    fn name(&self) -> &str {
        "时间戳转换"
    }

    fn description(&self) -> &str {
        "将时间戳转换为可阅读模式，或反向转换"
    }

    fn update(&mut self, ui: &mut Ui) {
        let mut responses = vec![];
        ui.horizontal(|ui| {
            responses.push(ui.text_edit_singleline(&mut self.input));
            if ui.add(Button::new("❌").frame(false)).clicked() {
                self.input = String::new();
                self.format_warning = "";
                responses[0].request_focus();
            }
            ui.add(Label::new(
                RichText::new(self.format_warning).color(Color32::YELLOW),
            ));
        });
        ui.horizontal(|ui| {
            if ui
                .add(Button::new(&self.converted).frame(false))
                .on_hover_text("点击复制")
                .clicked()
            {
                ui.ctx().copy_text(self.converted.clone());
                self.copied_prompt = "已复制";
            }
            ui.add(Label::new(
                RichText::new(self.copied_prompt).color(Color32::DARK_GREEN),
            ));
        });
        ui.horizontal(|ui| {
            responses.extend([
                ui.selectable_value(&mut self.unit, TimeUnit::Sec, "秒"),
                ui.selectable_value(&mut self.unit, TimeUnit::Milli, "毫秒"),
                ui.selectable_value(&mut self.unit, TimeUnit::Micro, "微秒"),
                ui.selectable_value(&mut self.unit, TimeUnit::Nano, "纳秒"),
            ]);
        });
        if responses.iter().any(|r| r.changed()) {
            let input = self.input.trim();
            self.copied_prompt = "";
            let mut input_type = InputType::Invalid;
            if input.is_empty() {
                self.converted.clear();
            } else if input.len() > 10 {
                let (s, r) = input.split_at(10);
                if let (Ok(secs), Ok(rr)) = (s.parse::<i64>(), r.parse::<u32>()) {
                    input_type = InputType::Timestamp;
                    let nsecs = rr * 10_u32.pow(9 - r.len() as u32);
                    //let dt = NaiveDateTime::from_timestamp(secs, nsecs);
                    match Local.timestamp_opt(secs, nsecs) {
                        LocalResult::Single(dt) | LocalResult::Ambiguous(dt, _) => {
                            self.converted = match self.unit {
                                TimeUnit::Sec => dt.format(DT_FORMAT_S).to_string(),
                                TimeUnit::Milli => dt.format(DT_FORMAT_MS).to_string(),
                                TimeUnit::Micro => dt.format(DT_FORMAT_US).to_string(),
                                TimeUnit::Nano => dt.format(DT_FORMAT_NS).to_string(),
                            };
                        }
                        _ => self.format_warning = "时间解析失败",
                    }
                } else if let Ok(dt) = NaiveDateTime::parse_from_str(input, DT_FORMAT_MS) {
                    input_type = InputType::DateTimeStr;
                    match Local.from_local_datetime(&dt) {
                        LocalResult::Single(dt) => {
                            if let Some(ns) = dt.timestamp_nanos_opt() {
                                let scale = match self.unit {
                                    TimeUnit::Sec => 1e9 as i64,
                                    TimeUnit::Milli => 1e6 as i64,
                                    TimeUnit::Micro => 1e3 as i64,
                                    TimeUnit::Nano => 1,
                                };
                                self.converted = format!("{}", ns / scale);
                            } else {
                                self.format_warning = "时间解析失败";
                            }
                        }
                        _ => self.format_warning = "无法转换时间戳到本地时区",
                    }
                }
            } else if let Ok(secs) = input.parse::<i64>() {
                // len <= 10
                input_type = InputType::Timestamp;
                //let dt = NaiveDateTime::from_timestamp(secs, 0);
                match Local.timestamp_opt(secs, 0) {
                    LocalResult::Single(dt) | LocalResult::Ambiguous(dt, _) => {
                        self.converted = dt.format(DT_FORMAT_S).to_string();
                    }
                    LocalResult::None => self.format_warning = "时间解析失败",
                }
            }
            let len = input.len();
            self.format_warning = match input_type {
                InputType::Timestamp if len != 10 && len != 13 && len != 16 && len != 19 => {
                    "⚠ 非标准时间戳格式（秒/毫秒/微秒/纳秒）"
                }
                InputType::Invalid => "⚠ 无效输入",
                _ => "",
            };
        }
    }
}

impl Default for TimestampConverter {
    fn default() -> Self {
        let now = Local::now().naive_local();
        TimestampConverter {
            input: format!("{}", now.and_utc().timestamp_millis()),
            converted: now.format(DT_FORMAT_MS).to_string(),
            format_warning: "",
            copied_prompt: "",
            unit: TimeUnit::Milli,
        }
    }
}
