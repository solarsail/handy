use std::f32;

use eframe::egui::{
    Button, Color32, CursorIcon, RichText, ScrollArea, TextEdit, TextStyle, Ui, widgets::Label,
};
use egui_json_tree::{DefaultExpand, JsonTree, JsonTreeStyle, render::DefaultRender};
//use unescape::unescape;

#[derive(PartialEq, Eq)]
pub enum Formatter {
    None,
    Pretty,
    Minimize,
}

#[derive(PartialEq, Eq)]
pub enum Conversion {
    None,
    Serialize,
    Deserialize,
}

pub struct JsonConverter {
    input: String,
    converted: String,
    format: Formatter,
    conversion: Conversion,
    warning: String,
    use_json_tree: bool,
}

impl super::ToolItem for JsonConverter {
    fn name(&self) -> &str {
        "JSON 转换"
    }

    fn description(&self) -> &str {
        "JSON 字符串格式处理"
    }

    fn update(&mut self, ui: &mut Ui) {
        let bottom_height = 100.0;
        let available_height = ui.available_height() - bottom_height;
        let desired_height = available_height.max(300.0);
        let label_height = 30.0;
        let font_id = TextStyle::Monospace.resolve(ui.style());
        let line_height = ui.fonts(|fonts| fonts.row_height(&font_id));
        let input_rows = ((desired_height - label_height) / line_height).floor() as usize;

        ui.horizontal(|ui| {
            ui.set_min_height(desired_height);
            ui.columns(2, |col| {
                col[0].vertical(|ui| {
                    ui.label("输入");
                    ScrollArea::vertical()
                        .id_salt("input")
                        .auto_shrink([false; 2])
                        .show(ui, |ui| {
                            ui.add(
                                TextEdit::multiline(&mut self.input)
                                    .desired_width(f32::INFINITY)
                                    .desired_rows(input_rows)
                                    .code_editor(),
                            );
                        });
                });
                col[1].vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("转换结果");
                        if ui.button("复制").clicked() {
                            ui.ctx().copy_text(self.converted.clone());
                        }
                    });
                    ScrollArea::vertical()
                        .id_salt("converted")
                        .auto_shrink([false; 2])
                        .show(ui, |ui| {
                            if self.use_json_tree {
                                // 使用 json viewer
                                ui.add(|ui: &mut Ui| {
                                    ui.horizontal(|ui| {
                                        let value = serde_json::from_str::<serde_json::Value>(
                                            &self.converted,
                                        )
                                        .unwrap_or(serde_json::json!({}));
                                        JsonTree::new("json-tree", &value)
                                            .style(JsonTreeStyle::new().abbreviate_root(true))
                                            .default_expand(DefaultExpand::All)
                                            .on_render(|ui, context| {
                                                context
                                                    .render_default(ui)
                                                    .on_hover_cursor(CursorIcon::ContextMenu)
                                                    .context_menu(|ui| {
                                                        let pointer = context
                                                            .pointer()
                                                            .to_json_pointer_string();
                                                        if !pointer.is_empty()
                                                            && ui.button("复制路径").clicked()
                                                        {
                                                            ui.ctx().copy_text(format!(
                                                                "${}",
                                                                pointer.replace("/", ".")
                                                            ));
                                                            ui.close_menu();
                                                        }

                                                        if ui.button("复制值").clicked() {
                                                            if let Ok(pretty_str) =
                                                                serde_json::to_string_pretty(
                                                                    context.value(),
                                                                )
                                                            {
                                                                ui.ctx().copy_text(pretty_str);
                                                            }
                                                            ui.close_menu();
                                                        }
                                                    });
                                            })
                                            .show(ui);
                                    })
                                    .response
                                });
                            } else {
                                // 使用文本框
                                ui.add(
                                    TextEdit::multiline(&mut self.converted)
                                        .interactive(false)
                                        .desired_width(f32::INFINITY)
                                        .desired_rows(input_rows)
                                        .code_editor(),
                                );
                            }
                        });
                });
            });
        });
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            // 转换选项
            ui.label("转换：");
            ui.selectable_value(&mut self.conversion, Conversion::None, "无");
            ui.selectable_value(&mut self.conversion, Conversion::Deserialize, "反序列化");
            if ui
                .selectable_value(&mut self.conversion, Conversion::Serialize, "序列化")
                .clicked()
            {
                // 序列化后无法格式化 json
                self.format = Formatter::None;
            }
            ui.add_space(16.0);
            // 格式化选项
            ui.label("格式：");
            ui.selectable_value(&mut self.format, Formatter::None, "无");
            ui.selectable_value(&mut self.format, Formatter::Pretty, "格式化");
            ui.selectable_value(&mut self.format, Formatter::Minimize, "压缩");
        });
        ui.add_space(8.0);

        // 执行按钮
        let primary_btn_color = if ui.visuals().dark_mode {
            Color32::from_hex("#005c12").unwrap()
        } else {
            Color32::from_hex("#bbe19e").unwrap()
        };
        let btn_response = ui.scope(|ui| {
            ui.spacing_mut().button_padding = (8.0, 4.0).into();
            ui.add(Button::new("⚙ 处理").fill(primary_btn_color))
        });
        if btn_response.inner.clicked() {
            self.use_json_tree = false;
            match self.conversion {
                Conversion::Deserialize => match serde_json::from_str::<String>(&self.input) {
                    Ok(v) => self.converted = v,
                    Err(e) => self.warning = e.to_string(),
                },
                Conversion::Serialize => match serde_json::to_string(&self.input) {
                    Ok(v) => self.converted = v,
                    Err(e) => self.warning = e.to_string(),
                },
                _ => self.converted = self.input.clone(),
            }
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(&self.converted) {
                match self.format {
                    Formatter::Pretty => match serde_json::to_string_pretty(&v) {
                        Ok(c) => {
                            self.converted = c;
                            self.use_json_tree = true;
                        }
                        Err(e) => self.warning = e.to_string(),
                    },
                    Formatter::Minimize => match serde_json::to_string(&v) {
                        Ok(c) => self.converted = c,
                        Err(e) => self.warning = e.to_string(),
                    },
                    _ => {}
                }
            }
        }

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
            format: Formatter::Pretty,
            conversion: Conversion::None,
            warning: String::new(),
            use_json_tree: false,
        }
    }
}
