use crate::{define_filtered_qmp_stream, qmp::messages::QmpUnknown};


define_filtered_qmp_stream!(QmpUnknownStream, Unknown, QmpUnknown);