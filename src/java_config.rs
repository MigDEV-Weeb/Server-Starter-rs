use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SelectedJavaVersion {
    V8,
    V11,
    V17,
}

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

    pub fn versions(&self) -> &JavaVersions {
        #[cfg(all(target_arch = "x86_64", target_os = "windows"))]
        return &self.windows_x86_64;

        #[cfg(all(target_arch = "x86_64", target_os = "macos"))]
        return &self.macos_x86_64;

        #[cfg(all(target_arch = "x86_64", target_os = "linux"))]
        return &self.linux_x86_64;

        //aarch64

        #[cfg(all(target_arch = "aarch64", target_os = "windows"))]
        return &self.windows_aarch64;

        #[cfg(all(target_arch = "aarch64", target_os = "macos"))]
        return &self.macos_aarch64;

        #[cfg(all(target_arch = "aarch64", target_os = "linux"))]
        return &self.linux_aarch64;
    }
}

#[derive(Serialize, Deserialize)]
pub struct JavaVersions {
    pub java8: String,
    pub java11: String,
    pub java17: String
}

impl JavaVersions {
    pub fn get(&self, selected: SelectedJavaVersion) -> &str {
        match selected {
            SelectedJavaVersion::V8 => {
                &self.java8
            }
            SelectedJavaVersion::V11 => {
                &self.java11
            }
            SelectedJavaVersion::V17 => {
                &self.java17
            }
        }
    }
}
