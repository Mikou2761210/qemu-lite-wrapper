use futures::Stream;
use log::error;
use serde_json::Value;
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use tokio::io::AsyncRead;
use tokio_util::{
    codec::{FramedRead, LinesCodec},
    sync::CancellationToken,
};

use crate::qmp::messages::QmpMessage;

#[derive(Debug)]
pub struct QmpMessageStream<S>
where
    S: AsyncRead + Unpin + Send + 'static,
{
    framed: FramedRead<S, LinesCodec>,
    cancel: CancellationToken,
}

impl<S> QmpMessageStream<S>
where
    S: AsyncRead + Unpin + Send + 'static,
{
    pub fn new(stream: S, cancel: CancellationToken) -> Self {
        let framed = FramedRead::new(stream, LinesCodec::new());
        Self { framed, cancel }
    }

        pub fn cancel(&self) {
        self.cancel.cancel();
    }

    pub fn cancel_token(&self) -> CancellationToken {
        self.cancel.clone()
    }
}

impl<S> Stream for QmpMessageStream<S>
where
    S: AsyncRead + Unpin + Send + 'static,
{
    type Item = QmpMessage;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.cancel.is_cancelled() {
            return Poll::Ready(None);
        }

        let mut this = self.as_mut();
        loop {
            match futures::ready!(Pin::new(&mut this.framed).poll_next(cx)) {
                Some(Ok(line)) => match serde_json::from_str::<Value>(&line) {
                    Ok(val) => break Poll::Ready(Some(QmpMessage::from_value(val))),
                    Err(e) => {
                        error!("QmpMessageStream: parse error: {}", e);
                        // Skip invalid line and continue polling
                        continue;
                    }
                },
                Some(Err(e)) => {
                    error!("QmpMessageStream: read error: {}", e);
                    break Poll::Ready(None);
                }
                None => break Poll::Ready(None),
            }
        }
    }
}
