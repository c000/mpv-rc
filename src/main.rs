#![windows_subsystem = "windows"]
mod mpvrc;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(egui::vec2(960.0, 480.0)),
        ..Default::default()
    };
    eframe::run_native(
        &format!("{}: mpv Remote Controller v{}", PKG_NAME, PKG_VERSION),
        options,
        Box::new(|cc| {
            cc.egui_ctx.style_mut_of(egui::Theme::Dark, |style| {
                style.visuals.widgets.inactive.fg_stroke.color = egui::Color32::WHITE;
            });
            Ok(Box::new(mpvrc::App::default()))
        }),
    )
}
