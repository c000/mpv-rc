#![windows_subsystem = "windows"]
mod constants;
mod mpvrc;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(egui::vec2(960.0, 480.0)),
        ..Default::default()
    };
    eframe::run_native(
        &format!(
            "{}: mpv Remote Controller v{}",
            constants::PKG_NAME,
            constants::PKG_VERSION,
        ),
        options,
        Box::new(|cc| {
            cc.egui_ctx.style_mut_of(egui::Theme::Dark, |style| {
                style.visuals.widgets.inactive.fg_stroke.color = egui::Color32::WHITE;
            });
            Ok(Box::new(mpvrc::App::default()))
        }),
    )
}
