use std::{default, fmt::{Debug, Pointer}, time::Instant};

use analysis::{aster_score, chi_squared_score, friedman_key_length, ioc, kasiski_examination, key_elimination, match_percentage, percentage_blocks, phi_test, substitution_cipher_score};
use eframe::egui;

mod analysis;

mod crypt;
use crypt::*;
use egui::{style::HandleShape, Align, Button, FontId, IconPainter, Layout, Slider};

mod toolkit;
use toolkit::*;

mod kullback;
use kullback::*;

mod tigershark;
use tigershark::*;

mod bullshark;
use bullshark::*;

struct MyApp {
    encrypted: String,
    plaintext: String,
    key_length2: usize,
    key_length1: usize,  
    alphabet_key: String,
    key: String,
    terminal1: String, 
    screenWidth: f32,
    screenHeight: f32,
    editor_width: u8,
    last_update: Instant,
    permuations: usize,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            encrypted: "PLAINTEXT".to_string(),
            plaintext: "ENCRYPTED".to_string(),
            terminal1: "TERMINAL 1".to_string(),
            key_length2: 10,
            key_length1: 10, 
            alphabet_key: String::new(),
            key: String::new(),
            screenHeight: 1080.0,
            screenWidth: 1920.0,
            editor_width: 5,
            last_update: Instant::now(),
            permuations: 1000,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let now = Instant::now();
        
        egui::SidePanel::left("MAIN")
            .min_width(ctx.available_rect().width() * (0.06 * self.editor_width as f32))
            .resizable(false)
            .show(ctx, |ui| {

                let preset_options = vec![
                    ("Kryptos Section 1".to_string(), K1.to_string(), K1p.to_string()),
                    ("Kryptos Section 2".to_string(), K2.to_string(), K2p.to_string()),
                    ("Kryptos Section 3".to_string(), K3.to_string(), K3p.to_string()),
                    ("Kryptos Section 4".to_string(), K4.to_string(), K4p.to_string()),
                ];

                ui.label(egui::RichText::new("Editor Window Width").size(16.0));
                ui.add(egui::Slider::new(&mut self.editor_width, 1..=10));

                ui.horizontal(|ui| {
                    ui.add_sized(
                        [self.screenWidth * (0.03 * self.editor_width as f32), self.screenHeight * 0.75],
                        egui::TextEdit::multiline(&mut self.encrypted)
                            .text_color(egui::Color32::LIGHT_GREEN)
                            .font(FontId::monospace(16.0))
                            .hint_text("PLAINTEXT"),
                    );
                    ui.add_sized(
                        [self.screenWidth * (0.03 * self.editor_width as f32), self.screenHeight * 0.75],
                        egui::TextEdit::multiline(&mut self.plaintext)
                            .font(FontId::monospace(16.0))
                            .text_color(egui::Color32::LIGHT_GREEN)
                            .hint_text("ENCRYPTED"),
                    );
                    
                });

                let mut same_length = self.encrypted.len() == self.plaintext.len();
                ui.checkbox(&mut same_length, "Plaintext & Encrypted Equal Length");
                ui.add_space(self.screenHeight * 0.01);

                egui::ComboBox::from_label("~PRESETS~")
                    .selected_text(
                        preset_options
                            .iter()
                            .find(|&(_, v, _)| *v == self.plaintext)
                            .unwrap_or_else(|| &preset_options[0])
                            .0
                            .clone(),
                    )
                    .show_ui(ui, |ui| {
                        for (display, encrypted, plaintext) in &preset_options {
                            if ui.selectable_value(
                                &mut self.plaintext,
                                plaintext.clone(),
                                egui::RichText::new(display).size(16.0),
                            ).clicked() {
                                self.plaintext = plaintext.clone();
                                self.encrypted = encrypted.clone();
                            }
                        }
                    });
                ui.add_space(self.screenHeight * 0.01);
                ui.style_mut().spacing.slider_width = (self.screenWidth * (0.04 * self.editor_width as f32));
                ui.style_mut().spacing.slider_rail_height = 16.0;
                ui.add(egui::Slider::new(&mut self.key_length2, 1..=30).text("Key 2 Length"));
                ui.add_space(self.screenHeight * 0.01);
                ui.add_sized(
                    [self.screenWidth * (0.06 * self.editor_width as f32), self.screenHeight * 0.02],
                    egui::TextEdit::singleline(&mut self.alphabet_key)
                        .font(FontId::monospace(20.0))
                        .hint_text("Alphabet Key")
                        .text_color(egui::Color32::LIGHT_YELLOW),
                );

                ui.add_space(self.screenHeight * 0.01);

                ui.add_sized(
                    [self.screenWidth * (0.06 * self.editor_width as f32), self.screenHeight * 0.02],
                    egui::TextEdit::singleline(&mut self.key)
                        .font(FontId::monospace(20.0))
                        .hint_text("Key 2")
                        .text_color(egui::Color32::LIGHT_YELLOW),
                );
            });
            egui::SidePanel::left("ANALYSIS")
            .min_width(ctx.available_rect().width() * (0.06 * self.editor_width as f32))
            .resizable(false)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("AUTOMATED ANALYSIS");

                    ui.heading(egui::RichText::new(format!("CHI SCORE")).color(egui::Color32::LIGHT_GREEN).font(FontId::monospace(16.0)),
                        );
                    ui.heading(egui::RichText::new(format!("{}", chi_squared_score(&self.encrypted))).color(egui::Color32::LIGHT_GREEN).font(FontId::monospace(16.0)),
                        );
                    ui.heading(egui::RichText::new(format!("ENGLISH {} ENCRYPTED", percentage_blocks(chi_squared_score(&self.encrypted), 0.0, 10.0))).color(egui::Color32::LIGHT_GREEN).font(FontId::monospace(16.0)),
                        );
                    ui.separator();
                    ui.heading(egui::RichText::new(format!("MATCH SCORE")).color(egui::Color32::LIGHT_GREEN).font(FontId::monospace(16.0)),
                        );
                    ui.heading(egui::RichText::new(format!("{}", match_percentage(&self.encrypted, &self.plaintext))).color(egui::Color32::LIGHT_GREEN).font(FontId::monospace(16.0)),
                        );
                    ui.heading(egui::RichText::new(format!("NO MATCHES {} MATCHING", percentage_blocks(match_percentage(&self.encrypted, &self.plaintext), 0.0, 100.0))).color(egui::Color32::LIGHT_GREEN).font(FontId::monospace(16.0)),
                        );
                    ui.separator() ;
                    ui.heading(egui::RichText::new(format!("KASISKI EXAMINATION")).color(egui::Color32::LIGHT_GREEN).font(FontId::monospace(16.0)),
                        );
                    ui.heading(egui::RichText::new(format!("Likely Key Lengths")).color(egui::Color32::LIGHT_GREEN).font(FontId::monospace(16.0)),
                        );
                    ui.heading(egui::RichText::new(format!("KEYS LENGTHS: {:?}", kasiski_examination(&self.encrypted, &[1,2,4]))).color(egui::Color32::LIGHT_GREEN).font(FontId::monospace(16.0)),
                        );
                    ui.separator();
                    ui.heading(egui::RichText::new(format!("FRIEDMAN TEST")).color(egui::Color32::LIGHT_GREEN).font(FontId::monospace(16.0)),
                    );
                    ui.heading(egui::RichText::new(format!("{}", friedman_key_length(&self.encrypted, self.key_length2))).color(egui::Color32::LIGHT_GREEN).font(FontId::monospace(16.0)),
                    );
                     ui.heading(egui::RichText::new(format!("LOW CONFIDENCE {} HIGH CONFIDENCE", percentage_blocks(friedman_key_length(&self.encrypted, self.key_length2), 0.038, 0.068))).color(egui::Color32::LIGHT_GREEN).font(FontId::monospace(16.0)),
                    );
                    ui.separator();
                    ui.heading(egui::RichText::new(format!("INDEX OF COINCIDENCE")).color(egui::Color32::LIGHT_GREEN).font(FontId::monospace(16.0)),
                    );
                    ui.heading(egui::RichText::new(format!("{}", ioc(&self.encrypted))).color(egui::Color32::LIGHT_GREEN).font(FontId::monospace(16.0)),
                    );
                    ui.heading(egui::RichText::new(format!("RANDOM {} ENGLISH", percentage_blocks(ioc(&self.encrypted), 0.038, 0.068))).color(egui::Color32::LIGHT_GREEN).font(FontId::monospace(16.0)),
                    );
                    ui.separator();
                    ui.heading(egui::RichText::new(format!("KEY ELIMINATION")).color(egui::Color32::LIGHT_GREEN).font(FontId::monospace(16.0)),
                    );
                    ui.heading(egui::RichText::new(format!("Possible Key :{:?}", key_elimination(self.key_length2, &self.encrypted, &self.plaintext).2)).color(egui::Color32::LIGHT_GREEN).font(FontId::monospace(16.0)),
                    );
                    ui.heading(egui::RichText::new(format!("LOW CONFIDENCE {} HIGH CONFIDENCE", percentage_blocks(key_elimination(self.key_length2, &self.encrypted, &self.plaintext).1, 0.0, 0.6))).color(egui::Color32::LIGHT_GREEN).font(FontId::monospace(16.0)),
                    );
                    ui.separator();
                    ui.heading(egui::RichText::new(format!("Columnar Index Of Coincidence")).color(egui::Color32::LIGHT_GREEN).font(FontId::monospace(16.0)),
                    );
                    ui.heading(egui::RichText::new(format!("{}", phi_test(&self.encrypted, self.key_length2))).color(egui::Color32::LIGHT_GREEN).font(FontId::monospace(16.0)),
                    );
                    ui.heading(egui::RichText::new(format!("LOW CONFIDENCE {} HIGH CONFIDENCE", percentage_blocks(phi_test(&self.encrypted, self.key_length2), 1.0, 2.5))).color(egui::Color32::LIGHT_GREEN).font(FontId::monospace(16.0)),
                    );
                    ui.separator();
                    ui.heading(egui::RichText::new(format!("ASTER SCORE")).color(egui::Color32::LIGHT_GREEN).font(FontId::monospace(16.0)),
                    );
                    ui.heading(egui::RichText::new(format!("AVG. Char Distance Of Matching Indices \n{}", aster_score(&self.encrypted, &self.plaintext))).color(egui::Color32::LIGHT_GREEN).font(FontId::monospace(16.0)),
                    );
                    ui.heading(egui::RichText::new(format!("LOW MATCH {} CLOSE MATCH", percentage_blocks(aster_score(&self.encrypted, &self.plaintext), 0.0, 100.0))).color(egui::Color32::LIGHT_GREEN).font(FontId::monospace(16.0)),
                    );
                    ui.separator();
                    ui.heading(egui::RichText::new(format!("SUBSTITUTION SCORE")).color(egui::Color32::LIGHT_GREEN).font(FontId::monospace(16.0)),
                    );
                    ui.heading(egui::RichText::new(format!("Likelihood Of A Substituion")).color(egui::Color32::LIGHT_GREEN).font(FontId::monospace(16.0)),
                    );
                    ui.heading(egui::RichText::new(format!("LOW MATCH {} CLOSE MATCH", percentage_blocks(substitution_cipher_score(&self.encrypted, &self.plaintext).unwrap_or(0.0), 0.0, 100.0))).color(egui::Color32::LIGHT_GREEN).font(FontId::monospace(16.0)),
                    );
                    ui.separator();
                    
                });
            });
            egui::CentralPanel::default().show(ctx, |ui| {
                if ui.button(egui::RichText::new("Kullback Test\n             ").font(FontId::monospace(24.0))).clicked() {
                    self.terminal1 = kullback(&self.encrypted, self.key_length2)
                }
                if ui.button(egui::RichText::new("Bullshark Analysis\nViginere").font(FontId::monospace(24.0))).clicked() {
                    self.terminal1 = format!("Viginere\n{}\n\nBeaufort\n{}",bullshark_vigenere(&self.alphabet_key, &self.encrypted, &self.plaintext, self.key_length2), bullshark_beaufort(&self.alphabet_key, &self.encrypted, &self.plaintext, self.key_length2));
                }
                if ui.button(egui::RichText::new("Encrypt\nViginere").font(FontId::monospace(24.0))).clicked() {
                    self.terminal1 = vigenere_encrypt(&self.plaintext, &self.alphabet_key, Some(&self.key));
                }
                if ui.button(egui::RichText::new("Decrypt\nViginere").font(FontId::monospace(24.0))).clicked() {
                    self.terminal1 = vigenere_decrypt(&self.encrypted, &self.alphabet_key, Some(&self.key));
                }
                ui.add(egui::Slider::new(&mut self.key_length1, 1..=26).text("Key Length 1"));
                ui.add(egui::Slider::new(&mut self.permuations, 1..=100000).text("Permutations"));
                if ui.button(egui::RichText::new("Tiger Shark Vigenere").font(FontId::monospace(24.0))).clicked() {
                    self.terminal1 = tigershark_vigenere(self.key_length1, self.key_length2, &self.encrypted, &self.plaintext, self.permuations);
                }
                if ui.button(egui::RichText::new("Tiger Shark Beaufort").font(FontId::monospace(24.0))).clicked() {
                    self.terminal1 = tigershark_beaufort(self.key_length1, self.key_length2, &self.encrypted, &self.plaintext, self.permuations);
                }

                ui.label(egui::RichText::new(&self.terminal1).font(FontId::monospace(16.0)));
                
            });       
    }
}

fn main() -> Result<(), eframe::Error> {

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1920.0, 1080.0]),
        ..Default::default()
    };
    eframe::run_native(
        "KRYPTON",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::new(MyApp::default())
        }),
    )
}