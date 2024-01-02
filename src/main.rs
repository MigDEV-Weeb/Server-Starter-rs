#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

pub mod download_helper;
mod java_config;

use eframe::egui;
use poll_promise::Promise;


use egui::{Vec2, ViewportBuilder};
use crate::java_config::{JavaConfig, SelectedJavaVersion};

fn main() -> Result<(), eframe::Error> {
    let _java = JavaConfig::parse("java_version.json");
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size(Vec2::new(280f32, 200f32)),
        ..Default::default()
    };
    eframe::run_native(
        "Server Starter",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::<MyApp>::default()
        }),
    )
}

struct MyApp {
    version: String,
    java_version: SelectedJavaVersion,
    max_ram_usage: u32,
    initial_ram_usage: u32,

    current_download: Option<Promise<()>>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            version: "1.21.1".to_owned(),
            java_version: SelectedJavaVersion::V17,
            max_ram_usage: 2,
            initial_ram_usage: 1,
            current_download: None,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Manual Server Starter");
            ui.horizontal(|ui| {
                let name_label = ui.label("Version name: ");
                ui.text_edit_singleline(&mut self.version)
                    .labelled_by(name_label.id);
            });
            egui::ComboBox::from_label("Select one!")
                .selected_text(format!("Java {:?}", self.java_version))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.java_version, SelectedJavaVersion::V8, "Java 8");
                    ui.selectable_value(&mut self.java_version, SelectedJavaVersion::V11, "Java 11");
                    ui.selectable_value(&mut self.java_version, SelectedJavaVersion::V17, "Java 17");
                });
            ui.add(egui::Slider::new(&mut self.max_ram_usage, 1..=64).text("Max Ram Usage"));
            if self.initial_ram_usage > self.max_ram_usage {
                self.initial_ram_usage = self.max_ram_usage
            }
            ui.add(
                egui::Slider::new(&mut self.initial_ram_usage, 1..=64).text("Initial Ram Usage"),
            );
            if self.initial_ram_usage > self.max_ram_usage {
                self.max_ram_usage = self.initial_ram_usage
            }
            ui.label(format!(
                "MC Version '{}', Java Version {:?}",
                self.version, self.java_version
            ));

            let button_text = if self.current_download.is_some() {
                "Downloading"
            } else {
                "Start"
            };
            println!("{}", self.current_download.is_some());
            if ui.button(button_text).clicked() {
                if self.current_download.is_none() {
                    self.current_download = Some(Promise::spawn_thread("test", ||
                        download_helper::download("http://migdev.de/minecraft.zip", "zip")));
                }
            }

            if let Some(promise) = &self.current_download {
                if let Some(_request) = promise.ready() {
                    //DO_STH:
                    self.current_download = None;
                }
            }
        });
    }
}
