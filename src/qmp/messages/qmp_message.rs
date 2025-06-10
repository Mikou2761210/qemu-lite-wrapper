use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use serde_json::value::RawValue;

use crate::qmp::types::QmpId;
use super::{QmpError, QmpEvent, QmpGreeting, QmpKind, QmpPayload, QmpReply, QmpUnknown};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QmpMessage {
    Greeting(QmpGreeting),
    Event(QmpEvent),
    Reply(QmpReply),
    Error(QmpError),
    Unknown(QmpUnknown),
}
impl QmpMessage {
    pub fn from_value(value: Value) -> Self {
        match serde_json::from_value::<Self>(value.clone()) {
            Ok(msg) => msg,
            Err(err) => Self::Unknown(QmpUnknown {
                raw: value,
                error: Some(err.to_string()),
            }),
        }
    }
}
impl QmpPayload for QmpMessage {
    fn kind(&self) -> QmpKind {
        match self {
            QmpMessage::Greeting(_) => QmpKind::Greeting,
            QmpMessage::Event(_) => QmpKind::Event,
            QmpMessage::Reply(_) => QmpKind::Reply,
            QmpMessage::Error(_) => QmpKind::Error,
            QmpMessage::Unknown(_) => QmpKind::Unknown,
        }
    }

    fn id(&self) -> Option<&QmpId> {
        match self {
            QmpMessage::Reply(r) => r.id(),
            QmpMessage::Error(e) => e.id(),
            _ => None,
        }
    }

    fn as_raw_json(&self) -> Option<&RawValue> {
        match self {
            QmpMessage::Greeting(g) => g.as_raw_json(),
            QmpMessage::Event(e) => e.as_raw_json(),
            QmpMessage::Reply(r) => r.as_raw_json(),
            QmpMessage::Error(e) => e.as_raw_json(),
            QmpMessage::Unknown(u) => u.as_raw_json(),
        }
    }
}
