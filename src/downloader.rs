use std::fs::File;
use std::io::Write;
use std::path::Path;
use crate::java_config::JavaConfig;

pub fn download(url: &str, path: &str) {

    std::fs::create_dir_all(path).unwrap();

    let resp = reqwest::blocking::get(
        url,
    )
        .expect("Failed to download file");

    let body = resp.bytes().expect("Failed to get body from request");

    println!("{}", (format!("{}/{}", path, Path::new(url).file_name().unwrap().to_str().unwrap())));
    let mut file =
        File::create(format!("{}/{}", path, Path::new(url).file_name().unwrap().to_str().unwrap())).expect("Failed to create file");

    file.write(&body).expect("Failed to write bytes");

    let java = JavaConfig::parse("JavaVersion.json");

}
