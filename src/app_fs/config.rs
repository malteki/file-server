use std::{ fs, io::{ self, Write } };

use super::CONFIG_PATH;

#[derive(Debug, Clone, Hash, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub port: u16,
    pub fs_dir: String,
}

impl Default for Config {
    fn default() -> Self {
        Self { port: 1337, fs_dir: "./fs".to_string() }
    }
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    Ok(toml::from_str(&fs::read_to_string(CONFIG_PATH)?)?)
}

pub fn write_config(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    Ok(fs::write(CONFIG_PATH, toml::to_string_pretty(config)?)?)
}

pub fn write_config_if_not_exist(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    match fs::File::create_new(CONFIG_PATH) {
        Ok(mut file) => {
            let toml = toml::to_string_pretty(config)?;

            file.write_all(&toml.as_bytes())?;

            Ok(())
        }
        Err(err) => {
            if err.kind() == io::ErrorKind::AlreadyExists { Ok(()) } else { Err(Box::new(err)) }
        }
    }
}
