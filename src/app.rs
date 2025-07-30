use egui::{TextBuffer as _, mutex::RwLock};
use lazy_static::lazy_static;
use libloading::Library;
use log::{error, info};
use std::collections::BTreeMap;

use atlas_common::{CHashOptions, HashFlags, HashFunction, HashResult, RegisterFunc, Slice};

pub type ExternalRegistration = extern "C" fn(register: RegisterFunc);

lazy_static! {
    static ref HASHERS: RwLock<BTreeMap<String, (HashFlags, HashFunction)>> =
        RwLock::new(BTreeMap::new());
    static ref LIBRARIES: RwLock<Vec<Library>> = RwLock::new(Vec::new());
}

pub struct AtlasApp {
    label: String,
    hasher: String,
    seed: String,
    seed_int: u64,
    // secret: String,
}

impl Default for AtlasApp {
    fn default() -> Self {
        Self {
            label: String::new(),
            hasher: String::from("None"),
            seed: String::new(),
            seed_int: 0,
            // secret: String::new(),
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn register(name: *const i8, flags: HashFlags, hasher: HashFunction) {
    let name_str = unsafe { std::ffi::CStr::from_ptr(name) };
    let name_str = name_str.to_string_lossy().to_string();
    HASHERS.write().insert(name_str.clone(), (flags, hasher));
    info!("Registered plugin {name_str}");
}

fn register_plugins() -> anyhow::Result<()> {
    HASHERS.write().clear();
    LIBRARIES.write().clear();
    let current_exe = std::env::current_exe()?;
    let Some(exe_dir) = current_exe.parent() else {
        return Err(anyhow::anyhow!("Failed to find parent of current exe"));
    };
    let plugin_dir = exe_dir.join("plugins");
    for entry in std::fs::read_dir(plugin_dir)? {
        let entry = entry?;
        if entry
            .path()
            .extension()
            .is_some_and(|x| matches!(x.to_string_lossy().as_str(), "dll" | "so" | "dylib"))
        {
            let lib = unsafe { libloading::Library::new(entry.path())? };
            let fn_register = unsafe {
                lib.get::<ExternalRegistration>(b"register_hashers")?
                    .into_raw()
            };

            fn_register(register);
            LIBRARIES.write().push(lib);
        }
    }

    Ok(())
}

impl AtlasApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        if let Err(e) = register_plugins() {
            error!("Failed to register plugins: {e}");
        }
        Default::default()
    }
}

impl eframe::App for AtlasApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                    if ui.button("Refresh Plugins").clicked() {
                        if let Err(e) = register_plugins() {
                            error!("Failed to register plugins: {e}");
                        }
                    }
                });
                ui.add_space(16.0);
                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ComboBox::from_label("Hashing Function")
                .selected_text(&self.hasher)
                .show_ui(ui, |ui| {
                    for algo in HASHERS.read().keys() {
                        ui.selectable_value(&mut self.hasher, algo.clone(), algo);
                    }
                });

            ui.label("Input:");
            ui.text_edit_singleline(&mut self.label);

            // TODO: fix secrets
            // Empty u8 vec works, but not a vec with data?
            if self.hasher != "None" {
                let hashers_guard = HASHERS.read();
                let hash_function = hashers_guard.get(&self.hasher);
                if let Some((flags, func)) = hash_function {
                    // let mut secret_data = Vec::new();
                    // let mut secret_data_slice = Slice::<u8>::null();
                    let mut options = CHashOptions {
                        data: std::ptr::null(),
                    };

                    if flags.contains(HashFlags::SEEDED) {
                        ui.label("Seed: ");
                        ui.text_edit_singleline(&mut self.seed);
                        if let Ok(seed_int) = self.seed.parse::<u64>() {
                            self.seed_int = seed_int;
                        } else {
                            self.seed_int = 0;
                        }
                        options.data = Box::into_raw(Box::from(self.seed_int)).cast();
                    }

                    // if flags.contains(HashFlags::SECRET) {
                    //     ui.label("Secret (hex): ");
                    //     ui.text_edit_singleline(&mut self.secret);
                    //     // TODO: does this reference live to the end of the function?
                    //     if self.secret.starts_with("0x") {
                    //         self.secret = self.secret.trim_start_matches("0x").to_owned();
                    //     }
                    //     if let Ok(secret_vec) = hex::decode(self.secret.clone()) {
                    //         secret_data = secret_vec.clone();
                    //     } else {
                    //         secret_data = vec![0u8];
                    //     }
                    //     secret_data_slice = Slice::from_boxed_slice(secret_data.into_boxed_slice());
                    //     options.data = std::ptr::from_ref(&secret_data_slice).cast();
                    // }

                    let input_data = self.label.as_bytes();
                    let boxed = input_data.to_vec().into_boxed_slice();
                    let mut data: Slice<u8> = Slice::from_boxed_slice(boxed);
                    // SAFETY: options and data will never be null
                    let ret = unsafe {
                        func(
                            std::ptr::from_mut(&mut data),
                            std::ptr::from_ref(&options).cast(),
                        )
                    };
                    if ret != 0 {
                        ui.label(format!("ERROR: Internal error ocurred: {ret}."));
                    } else if let Some(hash) = data.into_boxed_slice() {
                        let hash_result = HashResult::from(hash);
                        let hash_label = ui.label(format!("Output: 0x{}", hash_result.0));
                        if hash_label.clicked() {
                            ctx.copy_text(hash_result.0);
                        } else if hash_label.secondary_clicked() {
                            ctx.copy_text(hash_result.flip_endian().0);
                        }
                    } else {
                        ui.label("ERROR: Result returned null!");
                    }
                } else {
                    ui.label("ERROR: Could not find the hash function!");
                }
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
