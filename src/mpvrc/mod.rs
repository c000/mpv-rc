mod addui;
mod client;
mod command;

#[derive(Default)]
pub struct App {
    addui: addui::AddUi,
    commands: Vec<command::Command>,
    bottom_status: String,

    clients: Vec<client::Client>,
    update_count: usize,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(if self.bottom_status.is_empty() {
                    &"mpv Remote Controller"
                } else {
                    self.bottom_status.as_str()
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                    egui::widgets::global_dark_light_mode_switch(ui);
                    ui.label(format!("{}", self.update_count));
                    self.update_count += 1;
                    ui.separator();
                });
            });
        });

        egui::SidePanel::left("left_panel")
            .default_width(0.0)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
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

                    ui.label("Commands");
                    self.commands.retain_mut(|c| {
                        let mut keep = true;
                        ui.group(|ui| {
                            ui.horizontal(|ui| {
                                ui.heading(c.title());
                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Min),
                                    |ui| {
                                        if ui.button("x").clicked() {
                                            keep = false;
                                        }
                                    },
                                );
                            });
                            c.ui(ui);
                        });
                        keep
                    });
                    ui.vertical_centered(|ui| {
                        if ui.button(" + ").clicked() {
                            self.commands.push(Default::default());
                        }
                    });
                });

                ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Organize windows").clicked() {
                            ui.ctx().memory_mut(|mem| mem.reset_areas());
                        }
                    })
                });
            });

        egui::CentralPanel::default().show(ctx, |_ui| {
            self.clients.retain_mut(|client| {
                let mut open = true;
                egui::Window::new(client.title())
                    .open(&mut open)
                    .show(ctx, |ui| {
                        client.ui(ui, &self.commands);
                    });
                open
            });
        });
    }
}
