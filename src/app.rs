use eframe::{
    egui::{self, FontData, FontDefinitions, FontFamily, Id, Sense, TextStyle},
    epi,
};

use crate::{
    tool_card::ToolCard,
    tools::{TimestampConverter, ToolItem},
};
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct App {
    #[cfg_attr(feature = "persistence", serde(skip))]
    tools: Vec<Box<dyn ToolItem>>,
    active_tool: Option<usize>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            // Example stuff:
            tools: vec![Box::new(TimestampConverter::default())],
            active_tool: Some(0),
        }
    }
}

impl epi::App for App {
    fn name(&self) -> &str {
        "Handy tools"
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        ctx: &egui::CtxRef,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
        // set fonts
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "PingFang".to_owned(),
            FontData::from_static(include_bytes!("../fonts/PingFangSC-Light.otf")),
        ); // .ttf and .otf supported
           // Put my font first (highest priority):
        fonts
            .fonts_for_family
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "PingFang".to_owned());
        fonts
            .family_and_size
            .insert(TextStyle::Heading, (FontFamily::Proportional, 24.0));
        fonts
            .family_and_size
            .insert(TextStyle::Body, (FontFamily::Proportional, 18.0));
        fonts
            .family_and_size
            .insert(TextStyle::Button, (FontFamily::Proportional, 18.0));
        ctx.set_fonts(fonts);
    }

    /// Called by the frame work to save state before shutdown.
    /// Note that you must enable the `persistence` feature for this to work.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        let Self { tools, active_tool } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            for (i, tool) in tools.iter().enumerate() {
                let mut card = ToolCard::new(tool.name(), tool.description());
                let response = ui
                    .scope(|ui| {
                        card.update(ui, active_tool.is_some() && active_tool.unwrap() == i);
                    })
                    .response;
                let response =
                    ui.interact(response.rect, Id::new("tool_card").with(i), Sense::click());
                if response.clicked() {
                    *active_tool = Some(i);
                }
            }
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to("eframe", "https://github.com/emilk/egui/tree/master/eframe");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.spacing_mut().window_padding = (16.0, 12.0).into();
            if let Some(idx) = active_tool {
                tools[*idx].update(ui);
            }
            egui::warn_if_debug_build(ui);
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }
    }
}
