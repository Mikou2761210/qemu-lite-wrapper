use crate::qmp::types::QmpId;
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, serde::Deserialize)]
pub struct QmpCommand {
    pub execute: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<QmpId>,
}

impl QmpCommand {
    pub fn new(execute: impl Into<String>) -> Self {
        Self {
            execute: execute.into(),
            arguments: None,
            id: None,
        }
    }

    pub fn with_arguments(mut self, args: Value) -> Self {
        self.arguments = Some(args);
        self
    }

    pub fn with_id(mut self, id: QmpId) -> Self {
        self.id = Some(id);
        self
    }
}
