use eframe::egui::Color32;

pub(crate) fn warn_color(dark_mode: bool) -> Color32 {
    if dark_mode {
        Color32::YELLOW
    } else {
        Color32::RED
    }
}

pub(crate) fn primary_color(dark_mode: bool) -> Color32 {
    if dark_mode {
        Color32::from_hex("#005c12").unwrap()
    } else {
        Color32::from_hex("#bbe19e").unwrap()
    }
}

pub(crate) fn prompt_color(dark_mode: bool) -> Color32 {
    if dark_mode {
        Color32::from_hex("#a3b18a").unwrap()
    } else {
        Color32::from_hex("#588157").unwrap()
    }
}

pub(crate) fn highlight_color(dark_mode: bool) -> Color32 {
    if dark_mode {
        Color32::from_hex("#ffa5ab").unwrap()
    } else {
        Color32::from_hex("#a53860").unwrap()
    }
}
