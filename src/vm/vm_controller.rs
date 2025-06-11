use tokio::io::{AsyncRead, AsyncWrite};

use super::VmInstance;

use crate::launcher::QemuLaunchArgs;
use crate::qmp::commands::{QmpCommand, QmpSendError, QmpSender};
use crate::qmp::streams::QmpMessageStream;

pub struct VmController<R, W>
where
    R: AsyncRead + Unpin + Send + 'static,
    W: AsyncWrite + Unpin + Send + 'static,
{
    instance: VmInstance,
    sender: Option<QmpSender<W>>,
    stream: Option<QmpMessageStream<R>>,
}

impl<R, W> VmController<R, W>
where
    R: AsyncRead + Unpin + Send + 'static,
    W: AsyncWrite + Unpin + Send + 'static,
{
    pub fn new(args: QemuLaunchArgs) -> VmController<R, W> {
        Self {
            instance: VmInstance::new(args),
            sender: None,
            stream: None,
        }
    }

    pub fn set_sender(&mut self, sender: Option<QmpSender<W>>) {
        self.sender = sender;
    }
    pub fn get_sender(&self) -> &Option<QmpSender<W>> {
        &self.sender
    }
    pub fn get_mut_sender(&mut self) -> &mut Option<QmpSender<W>> {
        &mut self.sender
    }

    pub fn set_message_stream(&mut self, stream: Option<QmpMessageStream<R>>) {
        self.stream = stream;
    }
    pub fn get_message_stream(&self) -> &Option<QmpMessageStream<R>> {
        &self.stream
    }
    pub fn get_mut_message_stream(&mut self) -> &mut Option<QmpMessageStream<R>> {
        &mut self.stream
    }

    pub async fn launch(&mut self) -> std::io::Result<()> {
        self.instance.launch().await
    }

    pub async fn terminate(&mut self) -> std::io::Result<()> {
        if let Some(stream) = &self.stream {
            stream.cancel();
        }
        self.instance.terminate().await
    }

    pub fn message_stream(&mut self) -> Option<&mut QmpMessageStream<R>> {
        self.stream.as_mut()
    }

    pub async fn send_command(&mut self, cmd: &QmpCommand) -> Result<(), QmpSendError> {
        match &mut self.sender {
            Some(sender) => sender.send(cmd).await,
            None => Err(QmpSendError::NotConnected),
        }
    }

    pub async fn system_powerdown(&mut self) -> Result<(), QmpSendError> {
        self.send_command(&QmpCommand::system_powerdown()).await
    }

    pub async fn quit(&mut self) -> Result<(), QmpSendError> {
        self.send_command(&QmpCommand::quit()).await
    }

    pub async fn reset(&mut self) -> Result<(), QmpSendError> {
        self.send_command(&QmpCommand::system_reset()).await
    }

    pub async fn pause(&mut self) -> Result<(), QmpSendError> {
        self.send_command(&QmpCommand::stop()).await
    }

    pub async fn resume(&mut self) -> Result<(), QmpSendError> {
        self.send_command(&QmpCommand::cont()).await
    }
}
