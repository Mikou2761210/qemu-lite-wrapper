use serde_json;
use tokio_util::codec::LinesCodecError;

#[derive(Debug)]
pub enum QmpSendError {
    Serialization(serde_json::Error),
    Codec(LinesCodecError),
    NotConnected,
}

impl std::fmt::Display for QmpSendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QmpSendError::Serialization(e) => write!(f, "Serialization error: {}", e),
            QmpSendError::Codec(e) => write!(f, "Codec error: {}", e),
            QmpSendError::NotConnected => write!(f, "QMP not connected"),
        }
    }
}

impl std::error::Error for QmpSendError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            QmpSendError::Serialization(e) => Some(e),
            QmpSendError::Codec(e) => Some(e),
            QmpSendError::NotConnected => None,
        }
    }
}

impl From<serde_json::Error> for QmpSendError {
    fn from(e: serde_json::Error) -> Self {
        QmpSendError::Serialization(e)
    }
}

impl From<LinesCodecError> for QmpSendError {
    fn from(e: LinesCodecError) -> Self {
        QmpSendError::Codec(e)
    }
}
