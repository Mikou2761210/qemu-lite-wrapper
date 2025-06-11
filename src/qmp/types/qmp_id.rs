use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QmpId {
    Num(u64),
    Str(String),
    Other(serde_json::Value),
}
