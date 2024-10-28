use std::{ fs, io::{ self, Write } };

use crate::APPDATA_DIR;

use super::CONFIG_PATH;

#[derive(Debug, Clone, Hash, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Config {
    #[serde(default = "config_defaults::ipv4_addr")]
    pub ipv4_addr: String,

    #[serde(default = "config_defaults::port")]
    pub port: u16,

    /// the response text for GET "/"
    /// (status will alway be "OK")
    #[serde(default = "config_defaults::root_response")]
    pub root_response: String,

    #[serde(default = "config_defaults::fs_dir")]
    pub fs_dir: String,
}

mod config_defaults {
    pub(super) fn ipv4_addr() -> String {
        "0.0.0.0".to_string()
    }

    pub(super) fn port() -> u16 {
        8008
    }

    pub(super) fn root_response() -> String {
        "ONLINE".to_string()
    }

    pub(super) fn fs_dir() -> String {
        "./appdata/fs".to_string()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ipv4_addr: config_defaults::ipv4_addr(),
            port: config_defaults::port(),
            root_response: config_defaults::root_response(),
            fs_dir: config_defaults::fs_dir(),
        }
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

/// this function will:
///  1. create the assets folder if not existent
///  2. create assets/config.toml if not existent
///  3. load assets/config.toml
pub fn init_config() -> Result<Config, Box<dyn std::error::Error>> {
    let _ = fs::create_dir_all(APPDATA_DIR);

    write_config_if_not_exist(&Config::default())?;

    let config = load_config()?;

    write_config(&config)?;

    Ok(config)
}
