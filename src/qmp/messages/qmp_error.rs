use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use serde_json::value::RawValue;

use super::{QmpKind, QmpPayload};
use crate::qmp::types::QmpId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QmpError {
    #[serde(rename = "error")]
    pub error: QmpErrorInner,

    #[serde(default)]
    pub id: Option<QmpId>,

    #[serde(skip)]
    pub raw_json: Option<Box<RawValue>>,
}
impl QmpError {
    pub fn class(&self) -> &str {
        &self.error.class
    }

    pub fn desc(&self) -> &str {
        &self.error.desc
    }

    pub fn data(&self) -> Option<&Value> {
        self.error.data.as_ref()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QmpErrorInner {
    pub class: String,
    pub desc: String,

    #[serde(default)]
    pub data: Option<Value>,
}

impl QmpPayload for QmpError {
    fn kind(&self) -> QmpKind {
        QmpKind::Error
    }

    fn id(&self) -> Option<&QmpId> {
        self.id.as_ref()
    }

    fn as_raw_json(&self) -> Option<&RawValue> {
        self.raw_json.as_deref()
    }
}
