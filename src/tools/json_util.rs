use eframe::egui::{widgets::Label, Button, Color32, RichText, Ui};

#[derive(PartialEq, Eq)]
pub enum Formatter {
    None,
    Prettifier,
    Minimizer,
}
pub struct JsonConverter {
    input: String,
    converted: String,
    formatter: Formatter,
    warning: String,
}

impl super::ToolItem for JsonConverter {
    fn name(&self) -> &str {
        "Json 转换"
    }

    fn description(&self) -> &str {
        "将 json 字符串转义或反转义"
    }

    fn update(&mut self, ui: &mut Ui) {
        let mut responses = vec![];
        // 输入框
        ui.horizontal(|ui| {
            ui.text_edit_multiline(&mut self.input);
        });
        // 按钮
        ui.horizontal(|ui| {
            // 格式处理
            if ui.add(Button::new("格式处理（无转换）")).clicked() {
                if let Ok(v) = serde_json::from_str::<serde_json::Value>(&self.input) {
                    match self.formatter {
                        Formatter::Prettifier => match serde_json::to_string_pretty(&v) {
                            Ok(c) => self.converted = c,
                            Err(e) => self.warning = e.to_string(),
                        },
                        Formatter::Minimizer => {
                            // TODO: minimize
                            self.converted = self.input.clone()
                        }
                        _ => self.converted = self.input.clone(),
                    }
                }
            }
            // 反转义
            if ui.add(Button::new("反转义")).clicked() {}
            // 转义
            if ui.add(Button::new("转义")).clicked() {}
            // 格式选项
            responses.extend([
                ui.radio_value(&mut self.formatter, Formatter::None, "无"),
                ui.radio_value(&mut self.formatter, Formatter::Prettifier, "展开"),
                ui.radio_value(&mut self.formatter, Formatter::Minimizer, "压缩"),
            ]);
        });
        // 输出框
        ui.horizontal(|ui| {
            ui.text_edit_multiline(&mut self.converted);
        });
        // 警告提示
        ui.horizontal(|ui| {
            ui.add(Label::new(
                RichText::new(&self.warning).color(Color32::YELLOW),
            ));
        });
    }
}

impl Default for JsonConverter {
    fn default() -> Self {
        JsonConverter {
            input: String::new(),
            converted: String::new(),
            formatter: Formatter::Prettifier,
            warning: String::new(),
        }
    }
}
