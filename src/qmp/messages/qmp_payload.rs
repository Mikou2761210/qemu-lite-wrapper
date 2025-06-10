use serde_json::value::RawValue;

use super::qmp_kind::QmpKind;
use crate::qmp::types::QmpId;

pub trait QmpPayload {
    fn kind(&self) -> QmpKind;
    fn id(&self) -> Option<&QmpId>; // monitor コマンド ID 等
    fn as_raw_json(&self) -> Option<&RawValue>;
}
