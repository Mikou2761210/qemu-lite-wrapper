use futures::SinkExt;
use serde::Serialize;
use tokio::io::AsyncWrite;
use tokio_util::codec::{FramedWrite, LinesCodec};

use super::QmpSendError;

#[derive(Debug)]
pub struct QmpSender<W>
where
    W: AsyncWrite + Unpin + Send + 'static,
{
    framed: FramedWrite<W, LinesCodec>,
}

impl<W> QmpSender<W>
where
    W: AsyncWrite + Unpin + Send + 'static,
{
    pub fn new(writer: W) -> Self {
        let framed = FramedWrite::new(writer, LinesCodec::new());
        Self { framed }
    }

    pub async fn send<T>(&mut self, command: &T) -> Result<(), QmpSendError>
    where
        T: Serialize + ?Sized,
    {
        let json = serde_json::to_string(command)?;
        self.framed.send(json).await?;
        Ok(())
    }
}
