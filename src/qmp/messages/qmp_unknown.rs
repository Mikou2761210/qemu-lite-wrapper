use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use serde_json::value::RawValue;

use super::{QmpKind, QmpPayload};
use crate::qmp::types::QmpId;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct QmpUnknown {
    pub raw: Value,
    pub error: Option<String>,
}
impl QmpPayload for QmpUnknown {
    fn kind(&self) -> QmpKind {
        QmpKind::Unknown
    }

    fn id(&self) -> Option<&QmpId> {
        None
    }

    fn as_raw_json(&self) -> Option<&RawValue> {
        None
    }
}
