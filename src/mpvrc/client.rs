use std::io::{ErrorKind, Read, Write};

use anyhow::{Context, Result};
use interprocess::os::windows::named_pipe::DuplexBytePipeStream;
use serde_json::json;

pub struct Client {
    path: String,
    pipe: DuplexBytePipeStream,

    commands: Vec<CommandBuf>,
    status: String,
}

impl Client {
    pub fn new(path: &str) -> std::io::Result<Self> {
        let pipe = DuplexBytePipeStream::connect(path)?;
        pipe.set_nonblocking(true)?;

        Ok(Self {
            path: String::from(path),
            pipe,
            commands: vec![Default::default()],
            status: "Status".into(),
        })
    }

    pub fn title(&self) -> &str {
        &self.path
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("add").clicked() {
                self.commands.push(Default::default());
            }
        });
        ui.separator();

        self.ui_cmd(ui);

        match self.read() {
            Ok(l) => {
                if !l.is_empty() {
                    self.status = l
                }
            }
            Err(e) => self.status = format!("{:?}", e),
        }

        ui.label(&self.status);
    }

    fn ui_cmd(&mut self, ui: &mut egui::Ui) {
        let mut send_cmd: Option<serde_json::Value> = None;
        self.commands.retain_mut(|command| match command.ui(ui) {
            CommandBufResult::Nop => true,
            CommandBufResult::Send(cmd) => {
                send_cmd = Some(cmd);
                true
            }
        });
        if let Some(cmd) = send_cmd {
            self.write(&cmd);
        }
    }

    fn write(&mut self, v: &serde_json::Value) {
        let s = format!("{}\n", serde_json::to_string(v).unwrap());
        if let Err(e) = self.pipe.write_all(s.as_bytes()) {
            self.status = format!("{:?}", e)
        }
    }

    fn read(&mut self) -> Result<String> {
        let mut buf = vec![0; 1024];
        let n = self
            .pipe
            .read(&mut buf)
            .or_else(|e| match e.kind() {
                ErrorKind::BrokenPipe | ErrorKind::WouldBlock => Ok(0),
                _ => Err(e),
            })
            .context("can't read")?;
        buf.truncate(n);
        String::from_utf8(buf).context("invalid utf8")
    }
}

struct CommandBuf {
    id: usize,
    button_label: String,
    len: usize,
    cmds: Vec<String>,
}

impl Default for CommandBuf {
    fn default() -> Self {
        Self {
            id: rand::random(),
            button_label: "SEND".into(),
            len: 2,
            cmds: vec!["show-text".into(), "from mpv-rc".into()],
        }
    }
}

enum CommandBufResult {
    Nop,
    Send(serde_json::Value),
}

impl Default for CommandBufResult {
    fn default() -> Self {
        CommandBufResult::Nop
    }
}

impl CommandBuf {
    fn ui(&mut self, ui: &mut egui::Ui) -> CommandBufResult {
        let mut add = false;
        let mut del = false;

        let commands = egui::containers::CollapsingHeader::new("Config")
            .id_source(self.id)
            .default_open(true)
            .show_unindented(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Button label");
                    ui.centered_and_justified(|ui| ui.text_edit_singleline(&mut self.button_label));
                });

                for (i, a) in &mut self.cmds[..self.len].iter_mut().enumerate() {
                    ui.horizontal(|ui| {
                        ui.label(format! {"{}",i});
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

        ui.vertical_centered_justified(|ui| {
            if ui.button(&self.button_label).clicked() {
                CommandBufResult::Send(self.encode())
            } else {
                CommandBufResult::Nop
            }
        })
        .inner
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

    fn encode(&self) -> serde_json::Value {
        json!({ "command": self.cmds[..self.len] })
    }
}
