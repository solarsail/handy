use eframe::egui::{
    FontFamily, FontId, ScrollArea, TextEdit, TextFormat, TextStyle, Ui, text::LayoutJob,
    widgets::Label,
};
use once_cell::sync::Lazy;
use regex::Regex;

use crate::style;

#[derive(PartialEq, Eq)]
enum LineFormat {
    LF,
    CRLF,
}

pub struct LineFormatter {
    input: String,
    converted: LayoutJob,
    line_format: LineFormat,
}

impl super::ToolItem for LineFormatter {
    fn name(&self) -> &str {
        "堆栈字符串格式处理"
    }

    fn description(&self) -> &str {
        "将堆栈字符串重新分行，并高亮展示代码行"
    }

    fn update(&mut self, ui: &mut Ui) {
        let bottom_height = 40.0;
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
                                self.format(ui.visuals().dark_mode);
                            }
                        });
                });
                col[1].vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("转换结果");
                        if ui.button("复制").clicked() {
                            ui.ctx().copy_text(self.converted.text.clone());
                        }
                    });
                    ScrollArea::vertical()
                        .id_salt("converted")
                        .auto_shrink([false; 2])
                        .show(ui, |ui| {
                            ui.add(Label::new(self.converted.to_owned()));
                        });
                });
            });
        });
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing = (8.0, 8.0).into();
            ui.label("格式：");
            let btns = vec![
                ui.selectable_value(&mut self.line_format, LineFormat::LF, "LF(\\n)"),
                ui.selectable_value(&mut self.line_format, LineFormat::CRLF, "CRLF(\\r\\n)"),
            ];
            for btn in btns {
                if btn.changed() {
                    self.format(ui.visuals().dark_mode);
                }
            }
        });
    }
}

impl LineFormatter {
    fn format(&mut self, dark_mode: bool) {
        let target = match self.line_format {
            LineFormat::LF => "\\n",
            LineFormat::CRLF => "\\r\\n",
        };
        let mut result = self.input.replace(target, "\n");
        result = result.replace("\\t", "\t");
        let lines = result.lines().collect::<Vec<_>>();
        self.converted = LayoutJob::default();
        for line in lines {
            let tf = if is_code_line(line) {
                TextFormat {
                    color: style::highlight_color(dark_mode),
                    font_id: FontId {
                        family: FontFamily::Monospace,
                        size: 12.0,
                        ..Default::default()
                    },
                    ..Default::default()
                }
            } else {
                TextFormat {
                    font_id: FontId {
                        family: FontFamily::Monospace,
                        size: 12.0,
                        ..Default::default()
                    },
                    ..Default::default()
                }
            };
            self.converted.append(&format!("{}\n", line), 0.0, tf);
        }
    }
}

impl Default for LineFormatter {
    fn default() -> Self {
        LineFormatter {
            input: String::new(),
            converted: LayoutJob::default(),
            line_format: LineFormat::LF,
        }
    }
}

fn is_code_line(line: &str) -> bool {
    static LINE_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"\w+:\d+( \+\w+)?").unwrap());
    LINE_PATTERN.is_match(line)
}
