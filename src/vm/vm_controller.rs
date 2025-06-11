use std::path::Path;
use tokio::net::UnixStream;
use tokio_util::sync::CancellationToken;

use super::VmInstance;

use crate::launcher::QemuLaunchArgs;
use crate::qmp::commands::{QmpCommand, QmpSender};
use crate::qmp::streams::QmpMessageStream;

#[derive(Debug)]
pub struct VmController {
    instance: VmInstance,
    sender: Option<QmpSender<tokio::net::unix::OwnedWriteHalf>>,
    stream: Option<QmpMessageStream<tokio::net::unix::OwnedReadHalf>>,
    cancel: Option<CancellationToken>,
}

impl VmController {
    pub fn new(args: QemuLaunchArgs) -> Self {
        Self {
            instance: VmInstance::new(args),
            sender: None,
            stream: None,
            cancel: None,
        }
    }

    pub async fn connect_qmp(&mut self, path: impl AsRef<Path>) -> std::io::Result<()> {
        let stream = UnixStream::connect(path).await?;
        let (read_half, write_half) = stream.into_split();
        let cancel = CancellationToken::new();
        self.sender = Some(QmpSender::new(write_half));
        self.stream = Some(QmpMessageStream::new(read_half, cancel.clone()));
        self.cancel = Some(cancel);
        Ok(())
    }

    pub async fn launch(&mut self) -> std::io::Result<()> {
        self.instance.launch().await
    }

    pub async fn terminate(&mut self) -> std::io::Result<()> {
        if let Some(token) = &self.cancel {
            token.cancel();
        }
        self.instance.terminate().await
    }

    pub fn message_stream(
        &mut self,
    ) -> Option<&mut QmpMessageStream<tokio::net::unix::OwnedReadHalf>> {
        self.stream.as_mut()
    }

    pub async fn send_command(
        &mut self,
        cmd: &QmpCommand,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        match &mut self.sender {
            Some(sender) => sender.send(cmd).await,
            None => Err("QMP not connected".into()),
        }
    }

    pub async fn system_powerdown(
        &mut self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.send_command(&QmpCommand::system_powerdown()).await
    }

    pub async fn quit(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.send_command(&QmpCommand::quit()).await
    }

    pub async fn reset(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.send_command(&QmpCommand::system_reset()).await
    }

    pub async fn pause(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.send_command(&QmpCommand::stop()).await
    }

    pub async fn resume(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.send_command(&QmpCommand::cont()).await
    }
}
