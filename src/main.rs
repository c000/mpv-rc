mod mpvrc;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(640.0, 480.0)),
        ..Default::default()
    };
    eframe::run_native(
        "mpv Remote Controller",
        options,
        Box::new(|_cc| Box::new(mpvrc::App::default())),
    )
}
