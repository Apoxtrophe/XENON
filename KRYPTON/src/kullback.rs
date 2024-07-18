use std::{collections::HashMap, vec};

use egui_plot::{Bar, BarChart, Plot, PlotPoint, Text};

use crate::{OBJECT_LENGTH, OBJECT_SIZE, PANEL_SIZE, SCREEN_HEIGHT};


pub fn split_and_transform(s: &str, n: usize) -> Option<Vec<Vec<char>>> {
    if s.is_empty() || n == 0 {
        return None;
    }
    let chars: Vec<char> = s.chars().collect();
    let mut result = Vec::new();

    for chunk in chars.chunks(n) {
        if chunk.len() == n {
            result.push(chunk.to_vec());
        }
    }
    if result.is_empty() {
        return None;
    }
    let mut transformed = vec![vec![' '; result.len()]; n];
    for i in 0..n {
        for j in 0..result.len() {
            transformed[i][j] = result[j][i];
        }
    }

    Some(transformed)
}

pub fn kullback_ioc(input: Vec<Vec<char>>) -> (Vec<Vec<String>>, f64) {
    fn calculate_ioc(row: &Vec<char>) -> f64 {
        let mut freq = HashMap::new();
        let len = row.len();

        for &c in row {
            *freq.entry(c).or_insert(0) += 1;
        }

        let mut ioc = 0.0;
        for &count in freq.values() {
            ioc += count as f64 * (count as f64 - 1.0);
        }
        
        ioc / (len as f64 * (len as f64 - 1.0))
    }

    let mut string_matrix = vec![];
    let mut ioc_sum = 0.0;

    for row in input {
        let string_row: Vec<String> = row.iter().map(|&c| c.to_string()).collect();
        string_matrix.push(string_row);

        let ioc = calculate_ioc(&row);
        ioc_sum += ioc;
    }

    let avg_ioc = ioc_sum / string_matrix.len() as f64;
    (string_matrix, avg_ioc)
}

pub fn kullback(
    encrypted_text: &str,
) -> Vec<f64> {
    let mut aggr_ioc: Vec<f64> = vec![0.0; 60];

    for i in 1..60 {
        let transformed = split_and_transform(encrypted_text, i).unwrap_or_default();
        let result = kullback_ioc(transformed);
        aggr_ioc[i] = result.1;
    }

    aggr_ioc
}

pub fn plot_kullback(ui: &mut egui::Ui, aggr_ioc: Vec<f64>) {
    let mut bars = Vec::new();
    
    for (i, &v) in aggr_ioc.iter().enumerate() {
        bars.push(Bar::new(i as f64, v));
    }

    let chart = BarChart::new(bars).name("Kullback IoC");

    Plot::new("Kullback IoC Plot")
        .width(OBJECT_SIZE) // Set the width of the plot
        .height(OBJECT_LENGTH) // Set the height of the plot
        .show(ui, |plot_ui| {
            plot_ui.bar_chart(chart);

            for (i, &v) in aggr_ioc.iter().enumerate() {
                if v > 0.06 {
                    let text = Text::new(PlotPoint::new(i as f64, v), egui::RichText::new(format!("{}", i)).size(16.0).color(egui::Color32::YELLOW))
                    .highlight(true);
                plot_ui.text(text);
                }
            }
        });
}