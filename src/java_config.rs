use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct JavaConfig {
    pub linux_x86_64: JavaVersions,
    pub linux_aarch64: JavaVersions,
    pub windows_x86_64: JavaVersions,
    pub windows_aarch64: JavaVersions,
    pub macos_x86_64: JavaVersions,
    pub macos_aarch64: JavaVersions
}

impl JavaConfig {
    pub fn parse(config_path: &str) -> Self {
        let content = fs::read_to_string(config_path).unwrap();
        serde_json::from_str(&content).unwrap()
    }
}

#[derive(Serialize, Deserialize)]
pub struct JavaVersions {
    pub java8: String,
    pub java11: String,
    pub java17: String
}

