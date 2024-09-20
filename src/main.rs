#![windows_subsystem = "windows"]
mod mpvrc;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(egui::vec2(640.0, 480.0)),
        ..Default::default()
    };
    eframe::run_native(
        &format!("mpv Remote Controller v{}", VERSION),
        options,
        Box::new(|_cc| Ok(Box::new(mpvrc::App::default()))),
    )
}
