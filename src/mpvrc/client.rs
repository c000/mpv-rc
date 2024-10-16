use std::{
    fmt::Write as _,
    io::{ErrorKind, Read, Write},
};

use super::command;
use anyhow::{Context, Result};
use interprocess::os::windows::named_pipe::{pipe_mode::Bytes, DuplexPipeStream};

pub struct Client {
    path: String,
    pipe: DuplexPipeStream<Bytes>,

    status: String,
}

impl Client {
    pub fn new<P>(path: P) -> std::io::Result<Self>
    where
        P: Into<String>,
    {
        let path = path.into();
        let pipe = DuplexPipeStream::connect_by_path(path.as_str())?;
        pipe.set_nonblocking(true)?;

        Ok(Self {
            path,
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
        ui.vertical(|ui| {
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
                Err(e) => {
                    self.status.clear();
                    write!(self.status, "{:?}", e).unwrap()
                }
            }

            ui.label(&self.status);
        });
    }

    fn write(&mut self, v: &serde_json::Value) {
        let s = format!("{}\n", serde_json::to_string(v).unwrap());
        if let Err(e) = self.pipe.write_all(s.as_bytes()) {
            self.status.clear();
            write!(self.status, "{:?}", e).unwrap()
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
