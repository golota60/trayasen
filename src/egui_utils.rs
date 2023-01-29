#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui::{self, Label, Window};
use idasen::{MAX_HEIGHT, MIN_HEIGHT};

use crate::config_utils;

pub struct MyApp {
    pub name: String,
    pub height: u16,
    pub error_dialog_open: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Template name".to_owned(),
            height: 7000,
            error_dialog_open: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Add new position");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your profile name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.horizontal(|ui| {
                let name_label = ui.label("Desk height: ");
                ui.add(egui::Slider::new(&mut self.height, MIN_HEIGHT..=MAX_HEIGHT))
                    .labelled_by(name_label.id);
            });
            if self.error_dialog_open {
                Window::new("Error")
                    .collapsible(false)
                    .resizable(false)
                    .show(ctx, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.add(Label::new("Duplicate position name."));
                            ui.add(Label::new("Please provide a unique name."));
                            if ui.button("Ok").clicked() {
                                self.error_dialog_open = false;
                            }
                        });
                    });
            }

            ui.vertical_centered_justified(|ui| {
                if ui.button("Add").clicked() {
                    let new_name = self.name.clone();
                    let new_value = self.height;

                    let mut config = config_utils::get_config();

                    let is_duplicate = config
                        .saved_positions
                        .iter()
                        .find(|elem| elem.name == new_name);
                    match is_duplicate {
                        Some(_) => {
                            // Duplicate found
                            self.error_dialog_open = true;
                        }
                        None => {
                            // No duplicate
                            config.saved_positions.push(config_utils::Position {
                                name: new_name,
                                value: new_value,
                            });
                            config_utils::update_config(&config);

                            _frame.close();
                        }
                    }
                }
            })
        });
    }
}
