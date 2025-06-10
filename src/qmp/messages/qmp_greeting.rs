use serde::Deserialize;
use serde::Serialize;
use serde_json::value::RawValue;

use super::{QmpKind, QmpPayload};
use crate::qmp::types::QmpId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QmpGreeting {
    #[serde(rename = "QMP")]
    pub qmp: QmpGreetingInner,
    #[serde(skip)]
    pub raw_json: Option<Box<RawValue>>,
}
impl QmpGreeting {
    pub fn version(&self) -> &QmpVersion {
        &self.qmp.version
    }
    pub fn capabilities(&self) -> &Vec<String> {
        &self.qmp.capabilities
    }
}
impl QmpPayload for QmpGreeting {
    fn kind(&self) -> QmpKind {
        QmpKind::Greeting
    }
    fn id(&self) -> Option<&QmpId> {
        None
    }
    fn as_raw_json(&self) -> Option<&RawValue> {
        self.raw_json.as_deref()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QmpGreetingInner {
    pub version: QmpVersion,
    pub capabilities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QmpVersion {
    pub qemu: QmpSemver,
    pub package: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QmpSemver {
    pub major: u64,
    pub minor: u64,
    pub micro: u64,
}
