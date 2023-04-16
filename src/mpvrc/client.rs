use std::io::{ErrorKind, Read, Write};

use anyhow::{Context, Result};
use interprocess::os::windows::named_pipe::DuplexBytePipeStream;
use serde_json::json;

pub struct Client {
    path: String,
    pipe: DuplexBytePipeStream,

    command: CommandBuf,
    status: String,
}

impl Client {
    pub fn new(path: &str) -> std::io::Result<Self> {
        let pipe = DuplexBytePipeStream::connect(path)?;
        pipe.set_nonblocking(true)?;

        Ok(Self {
            path: String::from(path),
            pipe,
            command: Default::default(),
            status: "Status".into(),
        })
    }

    pub fn title(&self) -> &str {
        &self.path
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("+").clicked() {
                self.command.add()
            }
            if ui.button("-").clicked() {
                self.command.del()
            }
        });

        self.command.ui(ui);

        match self.read() {
            Ok(l) => {
                if !l.is_empty() {
                    self.status = l
                }
            }
            Err(e) => self.status = format!("{:?}", e),
        }

        ui.with_layout(
            egui::Layout::left_to_right(egui::Align::Min).with_main_justify(true),
            |ui| {
                if ui.button("SEND").clicked() {
                    self.write(&self.command.encode());
                };
            },
        );

        ui.label(&self.status);
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
    len: usize,
    cmds: Vec<String>,
}

impl Default for CommandBuf {
    fn default() -> Self {
        Self {
            len: 1,
            cmds: vec!["".into()],
        }
    }
}

impl CommandBuf {
    fn ui(&mut self, ui: &mut egui::Ui) {
        let mut add = false;
        let mut del = false;

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

    fn encode(&self) -> serde_json::Value {
        json!({ "command": self.cmds[..self.len] })
    }
}
