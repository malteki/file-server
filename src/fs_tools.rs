use std::{ fs, path::PathBuf };

use walkdir::WalkDir;

use crate::*;

pub async fn get_file_list() -> Vec<String> {
    let mut files: Vec<String> = WalkDir::new(FS_DIR)
        .follow_links(false)
        .follow_root_links(false)
        .into_iter()
        .filter_map(|entry| { entry.ok() })
        .filter(|entry| { entry.metadata().map_or(false, |metadata| { metadata.is_file() }) })
        .map(|entry| { entry.into_path() })
        .filter_map(|path| {
            path.strip_prefix(FS_DIR)
                .map(|path| { PathBuf::from(path) })
                .ok()
        })
        .filter_map(|path| { path.to_str().map(|str| { str.to_owned() }) })
        .collect();

    files.sort_by(|a, b| {
        match count_char_occurrences(a, '/').cmp(&count_char_occurrences(b, '/')) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Equal => a.cmp(b),
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
        }
    });

    files
}

pub async fn generate_file_list_html() -> Result<(), std::io::Error> {
    let files = get_file_list().await;

    println!("detected:");
    let mut href_lines = String::new();
    for file in &files {
        println!(" {file}");
        href_lines += &format!("{}<br>", href_line(&file));
    }

    fs::write(FILE_LIST_PATH, FILE_LIST_BASE.replace("<!--HREF-LINES-->", &href_lines))
}

fn href_line(file_name: &str) -> String {
    // let file_name = file_name.replace(" ", "%");
    format!("<a href=\"/file?{file_name}\">{file_name}</a>")
}

fn count_char_occurrences(s: &str, c: char) -> usize {
    s.chars()
        .filter(|&ch| ch == c)
        .count()
}
