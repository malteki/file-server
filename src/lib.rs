pub mod api;

pub mod filesystem;
pub const FILE_LIST_HTML_BASE: &'static str = include_str!("../appdata/file-list-base.html");

pub mod config;

// ---

pub const APPDATA_DIR: &'static str = "./appdata";

pub const FILE_LIST_HTML_PATH: &'static str = "./appdata/file-list.html";
pub const CONFIG_PATH: &'static str = "./appdata/config.toml";
