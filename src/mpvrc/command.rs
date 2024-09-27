use std::path::Path;

use anyhow::Context;
use serde::ser::SerializeMap;
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

impl serde::Serialize for Command {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut m = serializer.serialize_map(Some(2))?;
        m.serialize_entry("title", &self.title)?;
        m.serialize_entry("cmds", &self.cmds[..self.len])?;
        m.end()
    }
}

impl<'de> serde::Deserialize<'de> for Command {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct CommandVisitor;

        impl<'de> serde::de::Visitor<'de> for CommandVisitor {
            type Value = Command;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("map")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut title = Default::default();
                let mut cmds = vec![];

                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "title" => title = map.next_value()?,
                        "cmds" => cmds = map.next_value()?,
                        _ => (),
                    }
                }

                Ok(Command {
                    id: rand::random(),
                    title,
                    len: cmds.len(),
                    cmds,
                })
            }
        }

        deserializer.deserialize_map(CommandVisitor)
    }
}

impl Command {
    pub fn title(&self) -> String {
        if self.title.is_empty() {
            self.cmds[..self.len].join(" ")
        } else {
            self.title.to_string()
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) -> egui::CollapsingResponse<()> {
        let mut add = false;
        let mut del = false;

        let commands = egui::containers::CollapsingHeader::new("Config")
            .id_source(self.id)
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

        commands
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

pub fn load_from<P>(path: P) -> anyhow::Result<Vec<Command>>
where
    P: AsRef<Path>,
{
    let f = std::fs::File::open(path).context("can't open file")?;
    serde_json::from_reader(f).context("can't decode")
}

pub fn save_to<P>(path: P, value: &[Command]) -> anyhow::Result<()>
where
    P: AsRef<Path>,
{
    let f = std::fs::File::create(path).context("can't create filE")?;
    serde_json::to_writer_pretty(f, value)?;
    Ok(())
}
