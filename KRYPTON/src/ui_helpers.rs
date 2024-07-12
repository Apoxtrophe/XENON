use egui::{Color32, FontId, RichText, TextEdit};

use crate::{FONT, FONT_SIZE, HEADING, HEADING_SIZE, LABEL, PANEL_SIZE};

pub fn title(ui: &mut egui::Ui, text: &str) {
    ui.heading(
        RichText::new(text)
            .color(HEADING)
            .font(FontId::monospace(HEADING_SIZE)),
    );
    ui.separator();
}


pub fn heading(ui: &mut egui::Ui, label: &str) {
    ui.heading(
        RichText::new(label)
            .color(FONT)
            .font(FontId::monospace(FONT_SIZE)),
    );
}

pub fn heading_label(ui: &mut egui::Ui, label: &str) {
    ui.heading(
        RichText::new(label)
            .color(LABEL)
            .font(FontId::monospace(FONT_SIZE)),
    );
}

pub fn percentage_heading(ui: &mut egui::Ui, label1: &str, label2: &str, percentage: &str) {
    ui.heading(
        RichText::new(format!("{} {} {}", label1, percentage, label2))
            .color(FONT)
            .font(FontId::monospace(FONT_SIZE)),
    );
    ui.separator();
}

pub fn text_edit<'a>(text: &'a mut String, hint: &str) -> TextEdit<'a> {
    TextEdit::multiline(text)
        .font(FontId::monospace(FONT_SIZE))
        .text_color(Color32::LIGHT_GREEN)
        .hint_text(hint)
        .desired_width(PANEL_SIZE)
}

pub fn singleline_edit<'a>(text: &'a mut String, hint: &str) -> TextEdit<'a> {
    TextEdit::singleline(text)
        .font(FontId::monospace(HEADING_SIZE))
        .hint_text(hint)
        .text_color(HEADING)
        .desired_width(PANEL_SIZE)
}

