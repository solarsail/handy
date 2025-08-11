use eframe::egui::{
    Align, Frame, Layout, RichText, ScrollArea, TextEdit, TextStyle, Ui, Vec2, widgets::Label,
};

use crate::style;

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
        let bottom_height = 86.0;
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
        ui.separator();

        ui.allocate_ui_with_layout(
            (0.0, 32.0).into(), // 高度需要不小于 frame 的高度
            Layout::left_to_right(Align::Center),
            |ui| {
                ui.spacing_mut().item_spacing = (8.0, 8.0).into();

                Frame::new()
                    .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
                    .inner_margin(Vec2::new(8.0, 4.0))
                    .corner_radius(2)
                    .show(ui, |ui| {
                        ui.label("自动更新");
                    });
                ui.add_space(16.0);

                ui.label("转换");
                Frame::default()
                    .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
                    .inner_margin(4)
                    .corner_radius(6)
                    .show(ui, |ui| {
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
            },
        );
        ui.add_space(8.0);

        // 警告信息
        ui.add(Label::new(
            RichText::new(&self.warning).color(style::warn_color(ui.visuals().dark_mode)),
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
