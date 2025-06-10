use crate::{define_filtered_qmp_stream, qmp::messages::QmpEvent};

define_filtered_qmp_stream!(QmpEventStream, Event, QmpEvent);
