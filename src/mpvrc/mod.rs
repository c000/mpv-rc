mod addui;
mod client;
mod command;
mod global_keybind;
mod win;

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
                    let mut current_index = 0;
                    self.commands.retain_mut(|c| {
                        let mut keep = true;
                        let g = ui.group(|ui| {
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
                            c.ui(ui)
                        });

                        current_index += 1;
                        global_keybind::request_focus_on_ctrl(
                            current_index,
                            ui,
                            g.inner.header_response.id,
                            g.response.rect,
                        );

                        keep
                    });
                    ui.vertical_centered(|ui| {
                        if ui.button(" + ").clicked() {
                            self.commands.push(Default::default());
                        }
                    });

                    ui.horizontal(|ui| {
                        if ui.button("Load").clicked() {
                            if let Some(path) = win::get_open_file_name(Some(&win::FILTER_JSON)) {
                                match command::load_from(path) {
                                    Ok(v) => self.commands = v,
                                    Err(e) => self.bottom_status = format!("can't load {:?}", e),
                                }
                            }
                        }
                        if ui.button("Save").clicked() {
                            if let Some(path) = win::get_save_file_name(Some(&win::FILTER_JSON)) {
                                if let Err(e) = command::save_to(path, &self.commands) {
                                    self.bottom_status = format!("can't save {:?}", e);
                                }
                            }
                        }
                    })
                });
            });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(if self.bottom_status.is_empty() {
                    &"mpv Remote Controller"
                } else {
                    self.bottom_status.as_str()
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                    egui::widgets::global_dark_light_mode_switch(ui);
                    if ui.button("Organize windows").clicked() {
                        ui.ctx().memory_mut(|mem| mem.reset_areas());
                    }
                    ui.label(format!("{}", self.update_count));
                    self.update_count += 1;
                    ui.separator();
                });
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
