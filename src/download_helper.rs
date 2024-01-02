use std::fs::File;
use std::io::{Cursor, Write};
use std::path::{Path, PathBuf};
use egui::TextBuffer;

use crate::java_config::JavaConfig;

pub fn download(url: &str, path: &str) {
    std::fs::create_dir_all(path).unwrap();

    let resp = reqwest::blocking::get(
        url,
    )
        .expect("Failed to download file");

    let body = resp.bytes().expect("Failed to get body from request");

    let filename = Path::new(url).file_name().unwrap().to_str().unwrap();
    let iszip = filename.contains("zip");

    println!("{}", format!("{}/{}", path, filename));
    let mut file =
        File::create(format!("{}/{}", path, filename)).expect("Failed to create file");

    file.write(&body).expect("Failed to write bytes");

    if iszip {
        extract(&*format!("{} {}", path, filename), path);
    }

    let java = JavaConfig::parse("javaVersion.json");
}

pub fn extract(path_to_file: &str, path_to_destination: &str) {
    zip_extract::extract(Cursor::new(path_to_file), &*PathBuf::from(path_to_destination), false).expect("Failed to extract file");
}
