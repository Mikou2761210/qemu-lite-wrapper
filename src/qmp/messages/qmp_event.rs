use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use serde_json::value::RawValue;

use super::{QmpKind, QmpPayload};
use crate::qmp::types::{QmpId, QmpTimestamp};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QmpEvent {
    #[serde(rename = "event")]
    pub name: String,
    #[serde(default)]
    pub data: Option<Value>,
    #[serde(default)]
    pub timestamp: Option<QmpTimestamp>,
    #[serde(skip)]
    pub raw_json: Option<Box<RawValue>>,
}
impl QmpPayload for QmpEvent {
    fn kind(&self) -> QmpKind {
        QmpKind::Event
    }
    fn id(&self) -> Option<&QmpId> {
        None
    }
    fn as_raw_json(&self) -> Option<&RawValue> {
        self.raw_json.as_deref()
    }
}
