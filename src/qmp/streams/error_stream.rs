use crate::{define_filtered_qmp_stream, qmp::messages::QmpError};


define_filtered_qmp_stream!(QmpErrorStream, Error, QmpError);