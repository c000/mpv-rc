use crate::constants;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    default_command_path: String,

    pub initial_size_x: f32,
    pub initial_size_y: f32,
}

impl Config {
    pub fn command_path(&self) -> Option<&str> {
        match self.default_command_path.as_str() {
            "" => None,
            p => Some(p),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_command_path: Default::default(),
            initial_size_x: 960.0,
            initial_size_y: 480.0,
        }
    }
}

pub fn load() -> Result<Config, confy::ConfyError> {
    confy::load(constants::PKG_NAME, None)
}
