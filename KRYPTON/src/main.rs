use eframe::egui::{self, FontId};

mod analysis;
mod constants;
mod toolkit;
mod kullback;
mod tigershark;
mod bullshark;
mod ui_helpers;
mod vigenere;

use constants::*;
use egui::RichText;
use ui_helpers::*;
use analysis::*;
use toolkit::*;
use kullback::*;
use tigershark::*;
use bullshark::*;
use vigenere::*;

struct MyApp {
    encrypted: String,
    plaintext: String,
    key_length: usize,
    alphabet_key_length: usize,  
    alphabet_key: String,
    key: String,
    terminal1: String, 
    terminal2: String, 
    permuations: usize,
    vigenere_auto_encrypt: bool,
    vigenere_auto_decrypt: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            encrypted: "PLAINTEXT".to_string(),
            plaintext: "ENCRYPTED".to_string(),
            terminal1: "TERMINAL 1".to_string(),
            terminal2: "TERMINAL 2".to_string(),
            key_length: 10,
            alphabet_key_length: 10, 
            alphabet_key: String::new(),
            key: String::new(),
            permuations: 1000,
            vigenere_auto_encrypt: false,
            vigenere_auto_decrypt: false, 
        }
    }
}

impl MyApp {
    fn create_button(&mut self, ui: &mut egui::Ui, label: &str, callback: impl FnOnce() -> String) {
        if ui.button(RichText::new(label).font(FontId::monospace(HEADING_SIZE))).clicked() {
            self.terminal1 = callback();
        }       
    }

    fn update_terminal1(&mut self, value: String) {
        self.terminal1 = value;
    }

    fn update_terminal2(&mut self, value: String) {
        self.terminal2 = value;
    }
}

impl eframe::App for MyApp {            
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        
        
        // CHECKBOX AUTOMATED FUNCTIONS
        if self.vigenere_auto_decrypt && self.vigenere_auto_encrypt {
            self.vigenere_auto_decrypt = false;
            self.vigenere_auto_encrypt = false; 
        }
        
        if self.vigenere_auto_encrypt {
            self.encrypted = vigenere_encrypt(&self.plaintext, &self.alphabet_key, Some(&self.key));
        }
        
        if self.vigenere_auto_decrypt {
            self.plaintext = vigenere_decrypt(&self.encrypted, &self.alphabet_key, Some(&self.key));
        }
        
        egui::SidePanel::left("DASHBOARD")
            .max_width(PANEL_SIZE)
            .min_width(PANEL_SIZE)
            .resizable(false)
            .show(ctx, |ui| {
                let preset_options = vec![
                    ("Kryptos Section 1".to_string(), K1.to_string(), K1P.to_string()),
                    ("Kryptos Section 2".to_string(), K2.to_string(), K2P.to_string()),
                    ("Kryptos Section 3".to_string(), K3.to_string(), K3P.to_string()),
                    ("Kryptos Section 4".to_string(), K4.to_string(), K4P.to_string()),
                ];
                ui.vertical_centered(|ui| {
                    title(ui, "DASHBOARD");
                });
                ui.add_space(UI_SPACE);
                ui.style_mut().spacing.slider_width = PANEL_SIZE * 0.25;
                ui.style_mut().spacing.slider_rail_height = FONT_SIZE;


                ui.horizontal(|ui| {
                    ui.add(egui::Slider::new(&mut self.alphabet_key_length, 1..=MAX_KEY_LENGTH).text("ALPHABET KEY LENGTH"));
                    ui.add(egui::Slider::new(&mut self.key_length, 1..=MAX_KEY_LENGTH).text("KEY LENGTH"));                  
                });

                ui.add_space(UI_SPACE);
                ui.add_sized(
                    [PANEL_SIZE, HEADING_SIZE],
                    singleline_edit(&mut self.alphabet_key, "ALPHABET KEY")
                );

                ui.add_space(UI_SPACE);

                ui.add_sized(
                    [PANEL_SIZE, HEADING_SIZE],
                    singleline_edit(&mut self.key, "KEY")
                );
                ui.add_space(UI_SPACE);

                ui.horizontal(|ui| {
                    ui.add_sized(
                        [PANEL_SIZE * (0.5), SCREEN_HEIGHT * 0.75],
                        text_edit(&mut self.encrypted, "ENCRYPTED"),
                    );
                    ui.add_sized(
                        [PANEL_SIZE * (0.5), SCREEN_HEIGHT * 0.75],
                        text_edit(&mut self.plaintext, "PLAINTEXT"),
                    );             
                }); 
                ui.add_space(UI_SPACE);
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
                                egui::RichText::new(display).size(FONT_SIZE),
                            ).clicked() {
                                self.plaintext = plaintext.clone();
                                self.encrypted = encrypted.clone();
                            }
                        }
                    });
                ui.add_space(UI_SPACE); 
                let mut same_length = self.encrypted.len() == self.plaintext.len();
                ui.checkbox(&mut same_length, "Plaintext & Encrypted Equal Length");


            });
            egui::SidePanel::left("ANALYSIS")
            .max_width(PANEL_SIZE)
            .min_width(PANEL_SIZE)
            .resizable(false)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    title(ui, "AUTOMATED ANALYSIS");
                    
                    let chi_score = chi_squared_score(&self.encrypted);
                    let match_score = match_percentage(&self.encrypted, &self.plaintext);
                    let kasiski_score = kasiski_examination(&self.encrypted, &[1,2,4]);
                    let friedman_score = friedman_key_length(&self.encrypted, self.key_length);
                    let index_of_coincidence_score = ioc(&self.encrypted);
                    let key_elimination_score = key_elimination(self.key_length, &self.encrypted, &self.plaintext).2;
                    let columnar_coincidence_score = phi_test(&self.encrypted, self.key_length);
                    let aster_score = aster_score(&self.encrypted, &self.plaintext);
                    let substitution_score = substitution_cipher_score(&self.encrypted, &self.plaintext).unwrap_or(0.0);

                    let chi_percentage = percentage_blocks(chi_score, 0.0, 10.0);
                    let match_percentage = percentage_blocks(match_score, 0.0, 100.0);
                    let friedman_percentage = percentage_blocks(friedman_score, 0.038, 0.068);
                    let index_of_coincidence_percentage = percentage_blocks(index_of_coincidence_score, 0.038, 0.068);
                    let key_elimination_percentage = percentage_blocks(key_elimination(self.key_length, &self.encrypted, &self.plaintext).1, 0.0, 0.6);
                    let columnar_coincidence_percentage = percentage_blocks(columnar_coincidence_score, 1.0, 2.5);
                    let aster_percentage = percentage_blocks(aster_score, 0.0, 100.0);
                    let substitution_percentage = percentage_blocks(substitution_score, 0.0, 100.0);
                    
                    ui.add_space(UI_SPACE);
                    ui.group(|ui|{
                        heading(ui, "CHI SCORE");
                        heading_label(ui, &chi_score.to_string());
                        percentage_heading(ui, "ENGLISH", "ENCRYPTED", &chi_percentage);
                    });
                    
                    ui.add_space(UI_SPACE);
                    ui.group(|ui|{
                        heading(ui, "MATCH SCORE");
                        heading_label(ui, &match_score.to_string());
                        percentage_heading(ui, "NO MATCHES", "MATCHING", &match_percentage);
                    });
                    
                    ui.add_space(UI_SPACE);
                    ui.group(|ui|{
                        heading(ui, "KASISKI EXAMINATION");
                        ui.add_space(2.0 * UI_SPACE);
                        percentage_heading(ui, "", "", &format!("{:?}", kasiski_score));
                    });
                    
                    ui.add_space(UI_SPACE);
                    ui.group(|ui|{
                        heading(ui, "FRIEDMAN TEST");
                        heading_label(ui, &friedman_score.to_string());
                        percentage_heading(ui, "LOW CONFIDENCE", "HIGH CONFIDENCE", &friedman_percentage);
                    });
                   
                    ui.add_space(UI_SPACE);
                    ui.group(|ui|{
                        heading(ui, "INDEX OF COINCIDENCE");
                        heading_label(ui, &index_of_coincidence_score.to_string());
                        percentage_heading(ui, "RANDOM", "ENGLISH", &index_of_coincidence_percentage);
                    });
                   
                    ui.add_space(UI_SPACE);
                    ui.group(|ui|{
                        heading(ui, "KEY ELIMINATION");
                        heading_label(ui, &format!("Possible Key : {:?}", key_elimination_score));
                        percentage_heading(ui, "LOW CONFIDENCE", "HIGH CONFIDENCE", &key_elimination_percentage);
                    });
                    
                    ui.add_space(UI_SPACE);
                    ui.group(|ui|{
                        heading(ui, "COLUMNAR COINCIDENCE INDEX");
                        heading_label(ui, &columnar_coincidence_score.to_string());
                        percentage_heading(ui, "LOW CONFIDENCE", "HIGH CONFIDENCE", &columnar_coincidence_percentage);
                    });
                    
                    ui.add_space(UI_SPACE);
                    ui.group(|ui|{
                        heading(ui, "ASTER SCORE");
                        heading_label(ui, &aster_score.to_string());
                        percentage_heading(ui, "LOW MATCH", "CLOSE MATCH", &aster_percentage);
                    });
                    
                    ui.add_space(UI_SPACE);
                    ui.group(|ui|{
                        heading(ui, "SUBSTITUTION CIPHER SCORE");
                        heading_label(ui, &substitution_score.to_string());
                        percentage_heading(ui, "LOW MATCH", "CLOSE MATCH", &substitution_percentage);
                    });
                   
                    ui.add_space(UI_SPACE);
                    ui.group(|ui|{
                        heading(ui, "KULLBACK-LEIBLER DIVERGENCE");
                        let encrypted_kullback = &self.encrypted;
                        let aggr_ioc = kullback(&encrypted_kullback);
                        plot_kullback(ui, aggr_ioc);
                    }); 
                    ui.add_space(UI_SPACE);
                });
            });

            egui::TopBottomPanel::bottom("Output Terminal").show(ctx, |ui| {
                ui.vertical_centered(|ui|{
                    title(ui, "OUTPUT TERMINALS");
                });
                ui.horizontal(|ui|{
                    ui.add_sized(
                        (PANEL_SIZE, SCREEN_HEIGHT *0.25),
                        text_edit(&mut self.terminal1, "TERMINAL 1"));
                    ui.add_sized(
                        (PANEL_SIZE, SCREEN_HEIGHT *0.25),
                        text_edit(&mut self.terminal2, "TERMINAL 2"));
                });

            });

            egui::SidePanel::left("Encrypt")
            .resizable(false)
            .max_width(PANEL_SIZE * 0.5)
            .min_width(PANEL_SIZE * 0.5)
            .show(ctx, |ui|{
                ui.vertical_centered(|ui|{
                    title(ui, "ENCRYPT");
                    hint(ui, "OPERATE ON PLAINTEXT");
                    ui.group(|ui|{
                        ui.heading("VIGENERE");
                        ui.checkbox(&mut self.vigenere_auto_encrypt, "AUTO ENCRYPT");
                        create_button(ui, "ENCRYPT", 1.0, ||{
                            self.terminal1 = vigenere_encrypt(&self.plaintext, &self.alphabet_key, Some(&self.key));
                        });

                    });
                }); 
                
                
                
            });
            egui::SidePanel::left("Decode")
            .resizable(false)
            .max_width(PANEL_SIZE * 0.5)
            .min_width(PANEL_SIZE * 0.5)
            .show(ctx, |ui|{
                ui.vertical_centered(|ui|{
                    title(ui, "DECRYPT");
                    hint(ui, "OPERATE ON ENCRYPTED");
                    
                    ui.group(|ui| {
                        ui.heading("VIGENERE");
                        ui.checkbox(&mut self.vigenere_auto_decrypt, "AUTO DECRYPT");
                        create_button(ui, "DECRYPT", 1.0, ||{
                            let vigenere_decrypt = vigenere_decrypt(&self.encrypted, &self.alphabet_key, Some(&self.key));
                            self.terminal1 = vigenere_decrypt;
                        });
                    });
                });
            });
            egui::SidePanel::left("Decipher")
            .resizable(false)
            .max_width(PANEL_SIZE* 0.5)
            .min_width(PANEL_SIZE * 0.5)
            .show(ctx, |ui|{
                ui.vertical_centered(|ui|{
                    title(ui, "CRACK ENCRYPTION");
                    hint(ui, "OPERATE ON ENCRYPTED & PLAINTEXT");
                    ui.group(|ui|{
                       ui.heading("VIGENERE"); 
                       ui.add(egui::Slider::new(&mut self.permuations, 100..=100000).text("PERMUTATIONS"));
                       ui.add_space(UI_SPACE);
                       ui.horizontal(|ui|{
                           create_button(ui, "ReefShark", 3.0, ||{
                               
                           });
                           create_button(ui, "TigerShark", 3.0, ||{
                               
                           });
                           create_button(ui, "WhaleShark", 3.0, ||{
                               
                           });
                       });
                    });
                    
                });
            });
            egui::SidePanel::left("Transform")
            .resizable(false)
            .max_width(PANEL_SIZE * 0.5)
            .min_width(PANEL_SIZE * 0.5)
            .show(ctx, |ui|{
                ui.vertical_centered(|ui|{
                    title(ui, "TRANSFORM");
                    hint(ui, "OPERATE ON ENCRYPTED & PLAINTEXT");
                
                    create_button(ui, "SWAP TEXT", 1.0, ||{
                        let inter_plaintext = self.plaintext.clone();
                        let inter_encrypted = self.encrypted.clone();
                        
                        self.plaintext = inter_encrypted.to_string();
                        self.encrypted = inter_plaintext.to_string(); 
                    });
                });
                
                
            });

            /* 
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.vertical_centered(|ui|{

                    
                });

                
                


                if ui.button(egui::RichText::new("Bullshark Analysis\nViginere").font(FontId::monospace(HEADING_SIZE))).clicked() {
                    self.terminal1 = format!("Viginere\n{}\n\nBeaufort\n{}",bullshark_vigenere(&self.alphabet_key, &self.encrypted, &self.plaintext, self.key_length), bullshark_beaufort(&self.alphabet_key, &self.encrypted, &self.plaintext, self.key_length));
                }
                if ui.button(egui::RichText::new("Encrypt\nViginere").font(FontId::monospace(HEADING_SIZE))).clicked() {
                    self.terminal1 = vigenere_encrypt(&self.plaintext, &self.alphabet_key, Some(&self.key));
                }
                if ui.button(egui::RichText::new("Decrypt\nViginere").font(FontId::monospace(HEADING_SIZE))).clicked() {
                    self.terminal1 = vigenere_decrypt(&self.encrypted, &self.alphabet_key, Some(&self.key));
                }
                ui.add(egui::Slider::new(&mut self.permuations, 1..=100000).text("Permutations"));
                if ui.button(egui::RichText::new("Tiger Shark Vigenere").font(FontId::monospace(HEADING_SIZE))).clicked() {
                    self.terminal1 = tigershark_vigenere(self.alphabet_key_length, self.key_length, &self.encrypted, &self.plaintext, self.permuations);
                }
                if ui.button(egui::RichText::new("Tiger Shark Beaufort").font(FontId::monospace(HEADING_SIZE))).clicked() {
                    self.terminal1 = tigershark_beaufort(self.alphabet_key_length, self.key_length, &self.encrypted, &self.plaintext, self.permuations);
                }

                ui.label(egui::RichText::new(&self.terminal1).font(FontId::monospace(FONT_SIZE)));
                
            });  
            */    
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([SCREEN_WIDTH, SCREEN_HEIGHT]),
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