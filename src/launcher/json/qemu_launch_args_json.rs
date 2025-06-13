use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::launcher::QemuLaunchArgs;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct QemuLaunchArgsJson {
    #[serde(rename = "qemuLaunchArgs")]
    pub args: QemuLaunchArgs,
}
impl QemuLaunchArgsJson {
    pub fn new(args: QemuLaunchArgs) -> Self {
        Self { args: args }
    }

    pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn to_json_string_pretty(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn to_json_bytes(&self) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(self)
    }

    pub fn to_json_bytes_pretty(&self) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec_pretty(self)
    }

    pub fn from_json_str(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn from_json_bytes(bytes: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(bytes)
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P, pretty: bool) -> std::io::Result<()> {
        let content = if pretty {
            self.to_json_string_pretty().unwrap()
        } else {
            self.to_json_string().unwrap()
        };
        std::fs::write(path, content)
    }

    pub fn load_from_file<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&content).unwrap())
    }
}
