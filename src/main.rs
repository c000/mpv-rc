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
        Box::new(|cc| {
            cc.egui_ctx.style_mut_of(egui::Theme::Dark, |style| {
                style.visuals.override_text_color = Some(egui::Color32::WHITE);
            });
            Ok(Box::new(mpvrc::App::default()))
        }),
    )
}
