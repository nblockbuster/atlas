#![warn(clippy::all, rust_2018_idioms)]

fn main() -> eframe::Result {
    use env_logger::Env;
    env_logger::init_from_env(Env::new().default_filter_or("info")); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0])
            .with_icon(
                // NOTE: Adding an icon is optional
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
                    .expect("Failed to load icon"),
            ),
        ..Default::default()
    };
    eframe::run_native(
        "atlas",
        native_options,
        Box::new(|cc| Ok(Box::new(atlas::AtlasApp::new(cc)))),
    )
}
