use eframe::{
    Frame,
    egui::{
        self, Color32, FontData, FontFamily, FontId, Id, Layout, Sense, TextFormat, Ui,
        text::LayoutJob,
    },
    epaint::text::{FontInsert, FontPriority, InsertFontFamily},
};

use crate::{
    tool_card::ToolCard,
    tools::{JsonConverter, TimestampConverter, ToolItem, UrlConverter},
};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct App {
    #[cfg_attr(feature = "persistence", serde(skip))]
    tools: Vec<Box<dyn ToolItem>>,
    active_tool: Option<usize>,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        add_font(&cc.egui_ctx);
        Self {
            tools: vec![
                Box::new(TimestampConverter::default()),
                Box::new(JsonConverter::default()),
                Box::new(UrlConverter::default()),
            ],
            active_tool: Some(0),
        }
    }
}

impl eframe::App for App {
    /// Called by the frame work to save state before shutdown.
    /// Note that you must enable the `persistence` feature for this to work.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        let Self {
            tools,
            active_tool: active_tool_idx,
        } = self;
        // 标题栏
        if cfg!(target_arch = "wasm32") {
            egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.set_min_height(60.0);
                    ui.add_space(12.0);
                    let mut job = LayoutJob::default();
                    job.append(
                        "实用小工具",
                        0.0,
                        TextFormat {
                            font_id: FontId::proportional(24.0),
                            ..Default::default()
                        },
                    );
                    job.append(
                        "by ADSim Issue with ",
                        10.0,
                        TextFormat {
                            color: Color32::DARK_GRAY,
                            line_height: Some(22.0),
                            ..Default::default()
                        },
                    );
                    job.append(
                        "❤",
                        0.0,
                        TextFormat {
                            color: Color32::RED,
                            line_height: Some(22.0),
                            ..Default::default()
                        },
                    );
                    ui.label(job);

                    let ref_line = |ui: &mut Ui| {
                        ui.spacing_mut().item_spacing.x = 0.0;
                        ui.hyperlink_to("📦 repo ", "https://ad-gitlab.nioint.com/xinye.lu/handy");
                        ui.label("| Powered by ");
                        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                        ui.label(" and ");
                        ui.hyperlink_to(
                            "eframe",
                            "https://github.com/emilk/egui/tree/master/crates/eframe",
                        );
                    };
                    /*
                    let desired_size = ui.horizontal(|ui| ref_line(ui)).response.rect.size();
                    ui.label(format!("Desired size: {:?}", desired_size));
                    */

                    ui.allocate_ui_with_layout(
                        (218.4, 18.0).into(),
                        Layout::centered_and_justified(egui::Direction::LeftToRight),
                        |ui| {
                            ui.horizontal(|ui| {
                                ref_line(ui);
                            });
                        },
                    );
                    ui.add_space(8.0);
                });
            });
        }
        /*
        // 菜单栏
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            });
        });
        */
        // 边栏
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            for (i, tool) in tools.iter().enumerate() {
                // 选项卡
                let mut card = ToolCard::new(tool.name(), tool.description());
                // 使用局部样式更新
                let response = ui
                    .scope(|ui| {
                        card.update(
                            ui,
                            active_tool_idx.is_some() && active_tool_idx.unwrap() == i,
                        );
                    })
                    .response;
                // 检查点击
                let response =
                    ui.interact(response.rect, Id::new("tool_card").with(i), Sense::click());
                if response.clicked() {
                    *active_tool_idx = Some(i);
                }
                if i != tools.len() - 1 {
                    ui.separator();
                }
            }
            egui::warn_if_debug_build(ui);

            if cfg!(target_arch = "wasm32") {
                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.0;
                        ui.hyperlink_to("📦 repo ", "https://github.com/solarsail/handy");
                        ui.label("| powered by wasm");
                    });
                });
            }
        });

        // 主面板
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.spacing_mut().item_spacing = (16.0, 8.0).into();
            if let Some(idx) = active_tool_idx {
                tools[*idx].update(ui);
            }
        });
    }
}

fn add_font(ctx: &egui::Context) {
    ctx.add_font(FontInsert::new(
        "my_font",
        FontData::from_static(include_bytes!("../fonts/PingFangSC-Light.otf")),
        vec![
            InsertFontFamily {
                family: FontFamily::Proportional,
                priority: FontPriority::Highest,
            },
            InsertFontFamily {
                family: FontFamily::Monospace,
                priority: FontPriority::Lowest,
            },
        ],
    ));
}
