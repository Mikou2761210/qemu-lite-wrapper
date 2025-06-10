use crate::{define_filtered_qmp_stream, qmp::messages::QmpReply};


define_filtered_qmp_stream!(QmpReplyStream, Reply, QmpReply);