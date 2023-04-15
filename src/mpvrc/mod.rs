mod addui;
mod client;

#[derive(Default)]
pub struct App {
    addui: addui::AddUi,
    bottom_status: String,

    clients: Vec<client::Client>,
}

impl<'a> eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.label(if self.bottom_status.is_empty() {
                &"mpv Remote Controller"
            } else {
                self.bottom_status.as_str()
            });
        });

        egui::SidePanel::left("left_panel")
            .default_width(0.0)
            .show(ctx, |ui| {
                match self.addui.ui(ui) {
                    addui::AddUiResult::Add(path) => match client::Client::new(&path) {
                        Ok(c) => self.clients.push(c),
                        Err(e) => {
                            self.bottom_status = format!("{:?}", e);
                        }
                    },
                    addui::AddUiResult::Nop => (),
                }

                ui.separator();

                ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Organize windows").clicked() {
                            ui.ctx().memory_mut(|mem| mem.reset_areas());
                        }
                    })
                });
            });

        egui::CentralPanel::default().show(ctx, |_uii| {
            self.clients.retain_mut(|client| {
                let mut open = true;
                egui::Window::new(client.title())
                    .open(&mut open)
                    .show(ctx, |ui| {
                        client.ui(ui);
                    });
                open
            });
        });
    }
}
