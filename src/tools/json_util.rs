use std::f32;
#[cfg(not(target_arch = "wasm32"))]
use std::time::{Duration, Instant};

use eframe::egui::{
    Button, CursorIcon, FontFamily, FontId, RichText, ScrollArea, TextEdit, TextFormat, TextStyle,
    Ui, UiKind, text::LayoutJob, widgets::Label,
};
use egui_json_tree::{DefaultExpand, JsonTree, JsonTreeStyle, render::DefaultRender};

use crate::style;

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
    search_input: String,
    copied_prompt: &'static str,
    #[cfg(not(target_arch = "wasm32"))]
    prompt_vanish_at: Instant,
    pythonic_style: bool,
}

impl super::ToolItem for JsonConverter {
    fn name(&self) -> &str {
        "JSON 转换"
    }

    fn description(&self) -> &str {
        "JSON 字符串格式处理"
    }

    fn update(&mut self, ui: &mut Ui) {
        let bottom_height = 80.0;
        let available_height = ui.available_height() - bottom_height;
        let desired_height = available_height.max(300.0);
        let label_height = 26.0;
        let font_id = TextStyle::Monospace.resolve(ui.style());
        let line_height = ui.fonts(|fonts| fonts.row_height(&font_id));
        let input_rows = ((desired_height - label_height) / line_height).floor() as usize;
        #[cfg(not(target_arch = "wasm32"))]
        if self.copied_prompt != "" && Instant::now() > self.prompt_vanish_at {
            self.copied_prompt = "";
        }

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
                            self.copied_prompt = "已复制";
                            #[cfg(not(target_arch = "wasm32"))]
                            {
                                self.prompt_vanish_at = Instant::now() + Duration::from_secs(2);
                            }
                        }
                        ui.add(Label::new(
                            RichText::new(self.copied_prompt)
                                .color(style::prompt_color(ui.visuals().dark_mode)),
                        ));
                    });
                    if self.use_json_tree {
                        // 使用 json viewer
                        // 搜索框
                        let (text_edit_response, clear_button_response) = ui
                            .scope(|ui| {
                                ui.spacing_mut().item_spacing = (4.0, 4.0).into();
                                ui.horizontal(|ui| {
                                    ui.label("搜索：");
                                    let text_edit_response =
                                        ui.text_edit_singleline(&mut self.search_input);
                                    let clear_button_response =
                                        ui.add(Button::new("❌").frame(false));
                                    (text_edit_response, clear_button_response)
                                })
                                .inner
                            })
                            .inner;
                        ScrollArea::vertical()
                            .id_salt("converted")
                            .auto_shrink([false; 2])
                            .show(ui, |ui| {
                                ui.add(|ui: &mut Ui| {
                                    ui.vertical(|ui| {
                                        // json view
                                        let value = serde_json::from_str::<serde_json::Value>(
                                            &self.converted,
                                        )
                                        .unwrap_or(serde_json::json!({}));
                                        let response = JsonTree::new("json-tree", &value)
                                            .style(JsonTreeStyle::new().abbreviate_root(true))
                                            .default_expand(DefaultExpand::SearchResultsOrAll(
                                                &self.search_input,
                                            ))
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
                                                            ui.close_kind(UiKind::Menu);
                                                        }

                                                        if ui.button("复制值").clicked() {
                                                            if let Ok(pretty_str) =
                                                                serde_json::to_string_pretty(
                                                                    context.value(),
                                                                )
                                                            {
                                                                ui.ctx().copy_text(pretty_str);
                                                            }
                                                            ui.close_kind(UiKind::Menu);
                                                        }
                                                    });
                                            })
                                            .show(ui);

                                        if text_edit_response.changed() {
                                            response.reset_expanded(ui);
                                        }

                                        if clear_button_response.clicked() {
                                            self.search_input.clear();
                                            response.reset_expanded(ui);
                                        }
                                    })
                                    .response
                                });
                            });
                    } else {
                        // 使用文本框
                        ScrollArea::vertical()
                            .id_salt("converted")
                            .auto_shrink([false; 2])
                            .show(ui, |ui| {
                                ui.add(Label::new(
                                    RichText::new(&self.converted).text_style(TextStyle::Monospace),
                                ));
                                /*
                                ui.add(
                                    TextEdit::multiline(&mut self.converted)
                                        .interactive(false)
                                        .desired_width(f32::INFINITY)
                                        .desired_rows(input_rows)
                                        .code_editor(),
                                );
                                */
                            });
                    }
                });
            });
        });
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            // 执行按钮
            let btn_response = ui.scope(|ui| {
                ui.spacing_mut().button_padding = (8.0, 4.0).into();
                ui.add(Button::new("⚙ 处理").fill(style::primary_color(ui.visuals().dark_mode)))
            });

            ui.spacing_mut().item_spacing = (8.0, 8.0).into();
            if btn_response.inner.clicked() {
                self.copied_prompt = "";
                self.use_json_tree = false;
                let input = if self.pythonic_style {
                    self.input.replace("'", "\"").replace("None", "null")
                } else {
                    self.input.clone()
                };
                match self.conversion {
                    Conversion::Deserialize => match serde_json::from_str::<String>(&input) {
                        Ok(v) => self.converted = v,
                        Err(e) => self.warning = e.to_string(),
                    },
                    Conversion::Serialize => match serde_json::to_string(&input) {
                        Ok(v) => self.converted = v,
                        Err(e) => self.warning = e.to_string(),
                    },
                    _ => self.converted = input,
                }
                match serde_json::from_str::<serde_json::Value>(&self.converted) {
                    Ok(v) => match self.format {
                        Formatter::Pretty => match serde_json::to_string_pretty(&v) {
                            Ok(c) => {
                                self.converted = c;
                                self.use_json_tree = true;
                                self.warning = String::new();
                            }
                            Err(e) => self.warning = e.to_string(),
                        },
                        Formatter::Minimize => match serde_json::to_string(&v) {
                            Ok(c) => {
                                self.converted = c;
                                self.warning = String::new();
                            }
                            Err(e) => self.warning = e.to_string(),
                        },
                        _ => self.warning = String::new(),
                    },
                    Err(e) => self.warning = e.to_string(),
                }
            }

            // 转换选项
            ui.label("转换：");
            ui.selectable_value(&mut self.conversion, Conversion::None, "无");
            ui.selectable_value(&mut self.conversion, Conversion::Deserialize, "反序列化")
                .on_hover_ui(|ui| {
                    ui.label("输入需要为有效的字符串，包含两端的引号（\"）");
                });
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
            ui.selectable_value(&mut self.format, Formatter::Pretty, "展开");
            ui.selectable_value(&mut self.format, Formatter::Minimize, "压缩");
            ui.add_space(16.0);
            let mut cb_label_job = LayoutJob::default();
            cb_label_job.append("Python dict 风格（如 ", 0.0, TextFormat::default());
            cb_label_job.append(
                "{'key': None}",
                0.0,
                TextFormat {
                    font_id: FontId {
                        family: FontFamily::Monospace,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
            cb_label_job.append(" ）", 0.0, TextFormat::default());
            ui.checkbox(&mut self.pythonic_style, cb_label_job);
        });
        ui.add_space(8.0);

        // 警告提示
        ui.horizontal(|ui| {
            ui.add(Label::new(
                RichText::new(&self.warning).color(style::warn_color(ui.visuals().dark_mode)),
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
            search_input: String::new(),
            copied_prompt: "",
            #[cfg(not(target_arch = "wasm32"))]
            prompt_vanish_at: Instant::now(),
            pythonic_style: false,
        }
    }
}
