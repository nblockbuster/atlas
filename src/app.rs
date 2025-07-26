use std::usize;
use strum::IntoEnumIterator;

use crate::hashes::{HashingAlgorithm, flip_endian};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct AtlasApp {
    // Example stuff:
    label: String,
    hasher: HashingAlgorithm,
}

impl Default for AtlasApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: String::new(),
            hasher: HashingAlgorithm::Fnv1_32,
        }
    }
}

impl AtlasApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }
}

impl eframe::App for AtlasApp {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::MenuBar::new().ui(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            egui::ComboBox::from_label("Hashing Function")
                .selected_text(format!("{:?}", self.hasher))
                .show_ui(ui, |ui| {
                    for algo in HashingAlgorithm::iter() {
                        ui.selectable_value(&mut self.hasher, algo, format!("{algo:#?}"));
                    }
                });

            // ui.horizontal(|ui| {
            ui.label("Input:");
            ui.text_edit_singleline(&mut self.label);
            // });
            let hash_result = self.hasher.hasher()(self.label.as_bytes());
            let hash_label = ui.label(format!("Output: 0x{}", hash_result.0));
            if hash_label.clicked() {
                ctx.copy_text(hash_result.0);
            } else if hash_label.secondary_clicked() {
                ctx.copy_text(flip_endian(hash_result).0);
            }

            ui.separator();

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
