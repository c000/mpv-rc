use std::io::{ErrorKind, Read, Write};

use super::command;
use anyhow::{Context, Result};
use interprocess::os::windows::named_pipe::{pipe_mode::Bytes, DuplexPipeStream};

pub struct Client {
    path: String,
    pipe: DuplexPipeStream<Bytes>,

    status: String,
}

impl Client {
    pub fn new(path: &str) -> std::io::Result<Self> {
        let pipe = DuplexPipeStream::connect_by_path(path)?;
        pipe.set_nonblocking(true)?;

        Ok(Self {
            path: String::from(path),
            pipe,
            status: "Status".into(),
        })
    }

    pub fn title(&self) -> &str {
        &self.path
    }

    pub fn ui<'a, I>(&mut self, ui: &mut egui::Ui, commands: I)
    where
        I: IntoIterator<Item = &'a command::Command>,
    {
        ui.horizontal_wrapped(|ui| {
            for c in commands {
                if ui.button(c.title()).clicked() {
                    self.write(&c.encode());
                }
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
