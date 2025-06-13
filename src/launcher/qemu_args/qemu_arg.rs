use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum QemuArg {
    Flag(String),
    KeyValue(String, String),
    List(String, Vec<String>),
}
impl QemuArg {
    pub fn from_flag(flag: impl Into<String>) -> Self {
        QemuArg::Flag(flag.into())
    }

    pub fn from_key_value(key: impl Into<String>, value: impl Into<String>) -> Self {
        QemuArg::KeyValue(key.into(), value.into())
    }

    pub fn from_list(key: impl Into<String>, list: Vec<impl Into<String>>) -> Self {
        QemuArg::List(key.into(), list.into_iter().map(|s| s.into()).collect())
    }

    pub fn is_flag(&self) -> bool {
        matches!(self, QemuArg::Flag(_))
    }

    pub fn is_key_value(&self) -> bool {
        matches!(self, QemuArg::KeyValue(_, _))
    }

    pub fn is_list(&self) -> bool {
        matches!(self, QemuArg::List(_, _))
    }

    pub fn key(&self) -> &str {
        match self {
            QemuArg::Flag(flag) => flag,
            QemuArg::KeyValue(key, _) => key,
            QemuArg::List(key, _) => key,
        }
        .as_str()
    }

    pub fn key_equals(&self, key: &str) -> bool {
        self.key() == key
    }

    pub fn with_key(&self, new_key: impl Into<String>) -> Self {
        let new_key = new_key.into();
        match self {
            QemuArg::Flag(_) => QemuArg::Flag(new_key),
            QemuArg::KeyValue(_, value) => QemuArg::KeyValue(new_key, value.clone()),
            QemuArg::List(_, list) => QemuArg::List(new_key, list.clone()),
        }
    }

    pub fn to_args(&self) -> Vec<String> {
        match self {
            QemuArg::Flag(flag) => vec![flag.clone()],
            QemuArg::KeyValue(key, value) => vec![key.clone(), value.clone()],
            QemuArg::List(key, list) => {
                let joined = list.join(",");
                vec![key.clone(), joined]
            }
        }
    }

    pub fn to_command_line(&self) -> String {
        self.to_args()
            .iter()
            .map(|s| format!("\"{}\"", s.replace('"', "\\\"")))
            .collect::<Vec<_>>()
            .join(" ")
    }
}
