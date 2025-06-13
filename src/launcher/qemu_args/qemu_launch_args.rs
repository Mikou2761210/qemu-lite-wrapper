use serde::{Deserialize, Serialize};

use super::QemuArg;
#[derive(Debug, Clone, Hash, PartialEq, Eq, Default,Serialize,Deserialize)]
pub struct QemuLaunchArgs {
    #[serde(rename = "qemuBinary")]
    binary: String,
    #[serde(rename = "launchArguments")]
    args: Vec<QemuArg>,
    #[serde(rename = "positionalArgs")]
    positionals: Vec<String>,
}

impl QemuLaunchArgs {
    pub fn new(binary: impl Into<String>) -> Self {
        Self {
            binary: binary.into(),
            args: Vec::new(),
            positionals: Vec::new(),
        }
    }

    pub fn with_arg(mut self, arg: QemuArg) -> Self {
        self.args.push(arg);
        self
    }

    pub fn with_flag(mut self, flag: impl Into<String>) -> Self {
        self.args.push(QemuArg::from_flag(flag));
        self
    }

    pub fn with_key_value(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.args.push(QemuArg::from_key_value(key, value));
        self
    }

    pub fn with_list(mut self, key: impl Into<String>, list: Vec<impl Into<String>>) -> Self {
        self.args.push(QemuArg::from_list(key, list));
        self
    }

    pub fn with_args(mut self, args: impl IntoIterator<Item = QemuArg>) -> Self {
        self.args.extend(args);
        self
    }

    pub fn with_positional(mut self, value: impl Into<String>) -> Self {
        self.positionals.push(value.into());
        self
    }

    pub fn with_positionals(mut self, values: impl IntoIterator<Item = String>) -> Self {
        self.positionals.extend(values);
        self
    }

    pub fn parse_command_line(command_line: &str) -> Result<Self, String> {
        let tokens =
            shell_words::split(command_line).map_err(|e| format!("Tokenization failed: {}", e))?;

        if tokens.is_empty() {
            return Err("Empty command line".to_string());
        }

        let mut iter = tokens.into_iter();
        let binary = iter.next().unwrap();

        let mut args = Vec::new();
        let mut positionals = Vec::new();

        while let Some(token) = iter.next() {
            if token.starts_with('-') {
                if let Some(next) = iter.clone().next() {
                    if next.starts_with('-') {
                        args.push(QemuArg::from_flag(token));
                    } else if next.contains('=') || next.contains(',') {
                        iter.next();
                        let list = next
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .collect::<Vec<_>>();
                        args.push(QemuArg::from_list(token, list));
                    } else {
                        iter.next();
                        args.push(QemuArg::from_key_value(token, next));
                    }
                } else {
                    args.push(QemuArg::from_flag(token));
                }
            } else {
                positionals.push(token);
            }
        }

        Ok(QemuLaunchArgs {
            binary,
            args,
            positionals,
        })
    }

    pub fn get_binary(&self) -> &str {
        &self.binary
    }

    pub fn get_mut_binary(&mut self) -> &mut String {
        &mut self.binary
    }

    pub fn get_args(&self) -> &Vec<QemuArg> {
        &self.args
    }

    pub fn get_mut_args(&mut self) -> &mut Vec<QemuArg> {
        &mut self.args
    }

    pub fn get_positionals(&self) -> &Vec<String> {
        &self.positionals
    }

    pub fn get_mut_positionals(&mut self) -> &mut Vec<String> {
        &mut self.positionals
    }

    pub fn to_args(&self) -> Vec<String> {
        let mut result = vec![self.binary.clone()];
        for arg in &self.args {
            result.extend(arg.to_args());
        }
        result.extend(self.positionals.clone());
        result
    }

    pub fn to_command_line(&self) -> String {
        fn shell_escape(s: &str) -> String {
            if s.contains(' ') || s.contains('"') || s.contains(',') {
                format!("\"{}\"", s.replace('"', "\\\""))
            } else {
                s.to_string()
            }
        }

        self.to_args()
            .iter()
            .map(|s| shell_escape(s))
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub fn get_arg(&self, key: &str) -> Option<&QemuArg> {
        self.args.iter().find(|a| a.key_equals(key))
    }

    pub fn remove_arg(&mut self, key: &str) {
        self.args.retain(|a| !a.key_equals(key));
    }

    pub fn replace_arg(&mut self, new_arg: QemuArg) {
        self.remove_arg(new_arg.key());
        self.args.push(new_arg);
    }

    pub fn clear_positionals(&mut self) {
        self.positionals.clear();
    }

    pub fn remove_positional(&mut self, value: &str) {
        self.positionals.retain(|s| s != value);
    }

    pub fn remove_positional_at(&mut self, index: usize) -> Option<String> {
        if index < self.positionals.len() {
            Some(self.positionals.remove(index))
        } else {
            None
        }
    }
}
