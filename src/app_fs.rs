use std::fs::create_dir;

use config::{ load_config, write_config_if_not_exist, Config };

pub mod filesystem;
pub const FILE_LIST_HTML_BASE: &'static str = include_str!("../assets/file-list-base.html");

pub mod config;

// ---

pub const FS_DIR: &'static str = "./fs";
pub const ASSETS_DIR: &'static str = "./assets";

pub const FILE_LIST_HTML_PATH: &'static str = "./assets/file-list.html";
pub const CONFIG_PATH: &'static str = "./assets/config.toml";

// ---

/// this function will:
///  1. create the assets folder if not existent
///  2. create assets/config.toml if not existent
///  3. load assets/config.toml
pub fn init_fs() -> Result<Config, Box<dyn std::error::Error>> {
    let _ = create_dir(ASSETS_DIR);

    write_config_if_not_exist(&(Config { port: 1337, fs_dir: "./fs".to_string() }))?;

    let config = load_config()?;

    Ok(config)
}
