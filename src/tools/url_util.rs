use eframe::egui::{Color32, RichText, ScrollArea, TextEdit, TextStyle, Ui, widgets::Label};

#[derive(PartialEq, Eq)]
enum Conversion {
    Encode,
    Decode,
}

pub struct UrlConverter {
    input: String,
    converted: String,
    conversion: Conversion,
    warning: String,
}

impl super::ToolItem for UrlConverter {
    fn name(&self) -> &str {
        "URL 转换"
    }

    fn description(&self) -> &str {
        "URL 编解码处理"
    }

    fn update(&mut self, ui: &mut Ui) {
        let bottom_height = 80.0;
        let available_height = ui.available_height() - bottom_height;
        let desired_height = available_height.max(300.0);
        let label_height = 26.0;
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
                            if ui
                                .add(
                                    TextEdit::multiline(&mut self.input)
                                        .desired_width(f32::INFINITY)
                                        .desired_rows(input_rows)
                                        .code_editor(),
                                )
                                .changed()
                            {
                                self.convert();
                            }
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
                            ui.add(Label::new(
                                RichText::new(&self.converted).text_style(TextStyle::Monospace),
                            ));
                        });
                });
            });
        });
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            let btns = vec![
                ui.selectable_value(&mut self.conversion, Conversion::Decode, "解码"),
                ui.selectable_value(&mut self.conversion, Conversion::Encode, "编码"),
            ];
            for btn in btns {
                if btn.changed() {
                    self.convert();
                }
            }
        });
        ui.add_space(8.0);

        ui.add(Label::new(
            RichText::new(&self.warning).color(Color32::YELLOW),
        ));
    }
}

impl UrlConverter {
    fn convert(&mut self) {
        match self.conversion {
            Conversion::Encode => self.converted = urlencoding::encode(&self.input).into_owned(),
            Conversion::Decode => match urlencoding::decode(&self.input) {
                Ok(decoded) => self.converted = decoded.into(),
                Err(e) => self.warning = e.to_string(),
            },
        }
    }
}

impl Default for UrlConverter {
    fn default() -> Self {
        UrlConverter {
            input: String::new(),
            converted: String::new(),
            conversion: Conversion::Decode,
            warning: String::new(),
        }
    }
}
