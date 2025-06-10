use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum QmpId {
    Num(u64),
    Str(String),
    Other(serde_json::Value),
}
