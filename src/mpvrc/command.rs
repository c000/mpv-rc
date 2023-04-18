use serde_json::json;

pub struct Command {
    id: usize,
    title: String,
    len: usize,
    cmds: Vec<String>,
}

impl Default for Command {
    fn default() -> Self {
        Self {
            id: rand::random(),
            title: "".into(),
            len: 2,
            cmds: vec!["show-text".into(), "from mpv-rc".into()],
        }
    }
}

impl Command {
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        let mut add = false;
        let mut del = false;

        let commands = egui::containers::CollapsingHeader::new("Config")
            .id_source(self.id)
            .default_open(true)
            .show_unindented(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Title:");
                    ui.centered_and_justified(|ui| ui.text_edit_singleline(&mut self.title));
                });

                for (i, a) in &mut self.cmds[..self.len].iter_mut().enumerate() {
                    ui.horizontal(|ui| {
                        ui.label(format! {"{}:",i});
                        ui.centered_and_justified(|ui| {
                            let e = egui::TextEdit::singleline(a)
                                .hint_text("CTRL+I to add / CTRL+O to del")
                                .show(ui)
                                .response;

                            add |= e.has_focus()
                                && ui.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::I));

                            del |= e.has_focus()
                                && ui.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::O));
                        });
                    });
                }
            });

        commands.header_response.context_menu(|ui| {
            if ui.button("add").clicked() {
                add |= true;
            }
            if ui.button("del").clicked() {
                del |= true;
            }
        });

        if add {
            self.add();
        }

        if del {
            self.del();
        }
    }

    fn add(&mut self) {
        self.len += 1;
        if self.cmds.len() < self.len {
            self.cmds.resize(self.len, Default::default());
        }
    }

    fn del(&mut self) {
        if 1 < self.len {
            self.len -= 1;
        }
    }

    pub fn encode(&self) -> serde_json::Value {
        json!({ "command": self.cmds[..self.len] })
    }
}
