#![windows_subsystem = "windows"]
mod config;
mod constants;
mod mpvrc;

fn main() -> Result<(), eframe::Error> {
    let conf = config::load().unwrap_or_default();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(egui::vec2(conf.initial_size_x, conf.initial_size_y)),
        ..Default::default()
    };

    let mut app = mpvrc::App::default();
    if let Some(p) = conf.command_path() {
        app.load_command(p);
    }

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
            Ok(Box::new(app))
        }),
    )
}
