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
    clients_hovered: Vec<client::Client>,
    update_count: usize,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("left_panel")
            .default_width(0.0)
            .show(ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    ui.menu_button("File", |filemenu| {
                        if filemenu.button("Load").clicked() {
                            if let Some(path) = win::get_open_file_name(Some(&win::FILTER_JSON)) {
                                match command::load_from(path) {
                                    Ok(v) => self.commands = v,
                                    Err(e) => self.bottom_status = format!("can't load {:?}", e),
                                }
                            }
                            filemenu.close_menu();
                        }

                        if filemenu.button("Save").clicked() {
                            if let Some(path) = win::get_save_file_name(Some(&win::FILTER_JSON)) {
                                if let Err(e) = command::save_to(path, &self.commands) {
                                    self.bottom_status = format!("can't save {:?}", e);
                                }
                            }
                            filemenu.close_menu();
                        }
                    });
                });

                egui::ScrollArea::vertical().show(ui, |ui| {
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
                                        if ui.button("🗙").clicked() {
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

                    if ui.button("Add⏷").clicked() {
                        self.commands.push(Default::default());
                    }

                    ui.group(|ui| {
                        ui.heading("Connect");
                        match self.addui.ui(ui) {
                            addui::AddUiResult::Add(path) => match client::Client::new(path) {
                                Ok(c) => self.clients.push(c),
                                Err(e) => {
                                    self.bottom_status = format!("{:?}", e);
                                }
                            },
                            addui::AddUiResult::Nop => (),
                        }
                    });
                });
            });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(if self.bottom_status.is_empty() {
                    "mpv Remote Controller"
                } else {
                    self.bottom_status.as_str()
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                    egui::widgets::global_theme_preference_switch(ui);
                    if ui.button("Organize windows").clicked() {
                        ui.ctx().memory_mut(|mem| mem.reset_areas());
                    }
                    ui.label(format!("{}", self.update_count));
                    self.update_count += 1;
                    ui.separator();
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                let mut window_buttons = WindowButtons::new();
                ui.columns(self.clients.len(), |uis| {
                    for (i, c) in self.clients.iter_mut().enumerate() {
                        uis[i].group(|ui| {
                            ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    ui.heading(c.title());
                                    window_buttons.show_right_top_button(ui, i);
                                });
                                c.ui(ui, &self.commands);
                            });
                        });
                    }
                });
                window_buttons.modify_layouts(&mut self.clients, &mut self.clients_hovered);
            });

            {
                let mut window_buttons = WindowButtons::new();
                for (i, c) in self.clients_hovered.iter_mut().enumerate() {
                    egui::Window::new(c.title()).show(ctx, |ui| {
                        window_buttons.show_right_top_button(ui, i);
                        c.ui(ui, &self.commands);
                    });
                }
                window_buttons.modify_layouts(&mut self.clients_hovered, &mut self.clients);
            }
        });
    }
}

enum WindowButtons {
    Remove(usize),
    NextLayout(usize),
    Nop,
}

impl WindowButtons {
    pub fn new() -> Self {
        WindowButtons::Nop
    }

    pub fn show_right_top_button(&mut self, ui: &mut egui::Ui, index: usize) {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
            if ui.button("🗙").clicked() {
                *self = WindowButtons::Remove(index);
            }
            if ui.button("🗖").clicked() {
                *self = WindowButtons::NextLayout(index)
            }
        });
    }

    pub fn modify_layouts<T>(&self, current: &mut Vec<T>, next: &mut Vec<T>) {
        match self {
            WindowButtons::Remove(i) => {
                current.remove(*i);
            }
            WindowButtons::NextLayout(i) => {
                next.push(current.remove(*i));
            }
            WindowButtons::Nop => (),
        }
    }
}
