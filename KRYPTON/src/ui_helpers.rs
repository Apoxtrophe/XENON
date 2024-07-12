use egui::{Color32, FontId, RichText, TextEdit, Ui};

use crate::{DATA_COLOR, FONT_COLOR, FONT_SIZE, HEADING_COLOR, HEADING_SIZE, HINT_COLOR, LABEL_COLOR, PANEL_SIZE};

pub fn title(ui: &mut egui::Ui, text: &str) {
    ui.heading(
        RichText::new(text)
            .color(HEADING_COLOR)
            .font(FontId::monospace(HEADING_SIZE)),
    );
    ui.separator();
}


pub fn heading(ui: &mut egui::Ui, label: &str) {
    ui.heading(
        RichText::new(label)
            .color(FONT_COLOR)
            .font(FontId::monospace(FONT_SIZE)),
    );
}

pub fn hint(ui: &mut egui::Ui, label: &str) {
    ui.heading(
        RichText::new(label)
            .color(HINT_COLOR)
            .font(FontId::monospace(FONT_SIZE)),
    );
}


pub fn heading_label(ui: &mut egui::Ui, label: &str) {
    ui.heading(
        RichText::new(label)
            .color(LABEL_COLOR)
            .font(FontId::monospace(FONT_SIZE)),
    );
}

pub fn percentage_heading(ui: &mut egui::Ui, label1: &str, label2: &str, percentage: &str) {
    ui.heading(
        RichText::new(format!("{} {} {}", label1, percentage, label2))
            .color(DATA_COLOR)
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
        .text_color(HEADING_COLOR)
        .desired_width(PANEL_SIZE)
}

pub fn create_button(ui: &mut Ui, label: &str, on_click: impl FnOnce()) {
    if ui.button(RichText::new(label).font(FontId::monospace(HEADING_SIZE))).highlight().clicked() {
        on_click();
    }
}