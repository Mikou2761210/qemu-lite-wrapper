use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use serde_json::value::RawValue;

use super::{QmpKind, QmpPayload};
use crate::qmp::types::QmpId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QmpReply {
    #[serde(rename = "return")]
    pub result: Value,

    #[serde(default)]
    pub id: Option<QmpId>,

    #[serde(skip)]
    pub raw_json: Option<Box<RawValue>>,
}
impl QmpPayload for QmpReply {
    fn kind(&self) -> QmpKind {
        QmpKind::Reply
    }

    fn id(&self) -> Option<&QmpId> {
        self.id.as_ref()
    }

    fn as_raw_json(&self) -> Option<&RawValue> {
        self.raw_json.as_deref()
    }
}
