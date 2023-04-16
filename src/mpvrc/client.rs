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
        self.command.ui(ui);

        ui.horizontal(|ui| {
            if ui.button("+").clicked() {
                self.command.add()
            }
            if ui.button("-").clicked() {
                self.command.del()
            }
        });

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

#[derive(Default)]
struct CommandBuf {
    command: String,
    args: Vec<String>,
}

impl CommandBuf {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("0");
            ui.centered_and_justified(|ui| {
                ui.text_edit_singleline(&mut self.command);
            })
        });

        for (i, a) in &mut self.args.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                ui.label(format! {"{}",i+1});
                ui.text_edit_singleline(a);
            });
        }
    }

    fn add(&mut self) {
        self.args.push(Default::default());
    }

    fn del(&mut self) {
        self.args.pop();
    }

    fn encode(&self) -> serde_json::Value {
        let cmd = std::iter::once(self.command.as_str())
            .chain(self.args.iter().map(String::as_str))
            .collect::<Vec<&str>>();

        json!({ "command": cmd })
    }
}
