#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum QmpKind {
    Greeting,
    Event,
    Reply,
    Error,
    Unknown,
}