use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct QmpTimestamp {
    pub seconds: i64,
    #[serde(rename = "microseconds")]
    pub micros: i32,
}