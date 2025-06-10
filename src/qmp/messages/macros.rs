use super::qmp_error::QmpError;
use super::qmp_event::QmpEvent;
use super::qmp_message::QmpMessage;
use super::qmp_reply::QmpReply;
use super::qmp_unknown::QmpUnknown;

macro_rules! impl_tryfrom_qmp {
    ($ty:ident, $variant:ident) => {
        impl TryFrom<QmpMessage> for $ty {
            type Error = QmpMessage;
            fn try_from(m: QmpMessage) -> Result<Self, Self::Error> {
                if let QmpMessage::$variant(ev) = m {
                    Ok(ev)
                } else {
                    Err(m)
                }
            }
        }
    };
}
impl_tryfrom_qmp!(QmpEvent, Event);
impl_tryfrom_qmp!(QmpReply, Reply);
impl_tryfrom_qmp!(QmpError, Error);
impl_tryfrom_qmp!(QmpUnknown, Unknown);
