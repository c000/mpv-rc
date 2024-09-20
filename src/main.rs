#![windows_subsystem = "windows"]
mod mpvrc;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(egui::vec2(640.0, 480.0)),
        ..Default::default()
    };
    eframe::run_native(
        "mpv Remote Controller",
        options,
        Box::new(|_cc| Ok(Box::new(mpvrc::App::default()))),
    )
}
