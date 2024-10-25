pub mod api;

pub mod filesystem;
pub const FILE_LIST_HTML_BASE: &'static str = include_str!("../assets/file-list-base.html");

pub mod config;

// ---

pub const FS_DIR: &'static str = "./fs";
pub const ASSETS_DIR: &'static str = "./assets";

pub const FILE_LIST_HTML_PATH: &'static str = "./assets/file-list.html";
pub const CONFIG_PATH: &'static str = "./assets/config.toml";
