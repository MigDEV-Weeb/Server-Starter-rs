#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum JavaVersions {
    V8,
    V11,
    V17,
}
struct MyApp {
    version: String,
    java_version: JavaVersions,
    max_ram_usage: u32,
    initial_ram_usage: u32
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            version: "1.21.1".to_owned(),
            java_version: JavaVersions::V17,
            max_ram_usage: 2,
            initial_ram_usage: 1
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
                    ui.selectable_value(&mut self.java_version, JavaVersions::V8, "Java 8");
                    ui.selectable_value(&mut self.java_version, JavaVersions::V11, "Java 11");
                    ui.selectable_value(&mut self.java_version, JavaVersions::V17, "Java 17");
                });
            ui.add(egui::Slider::new(&mut self.max_ram_usage, 1..=64).text("Max Ram Usage"));
            if self.initial_ram_usage > self.max_ram_usage {self.initial_ram_usage = self.max_ram_usage}
            ui.add(egui::Slider::new(&mut self.initial_ram_usage, 1..=64).text("Initial Ram Usage"));
            if self.initial_ram_usage > self.max_ram_usage {self.max_ram_usage = self.initial_ram_usage}
            ui.label(format!("MC Version '{}', Java Version {:?}", self.version, self.java_version));

        });
    }
}