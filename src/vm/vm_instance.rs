use crate::launcher::{QemuLaunchArgs, QemuProcess};

#[derive(Debug)]
pub struct VmInstance {
    args: QemuLaunchArgs,
    process: Option<QemuProcess>,
}

impl VmInstance {
    pub fn new(args: QemuLaunchArgs) -> Self {
        Self {
            args,
            process: None,
        }
    }

    pub async fn launch(&mut self) -> std::io::Result<()> {
        let proc = QemuProcess::launch(&self.args).await?;
        self.process = Some(proc);
        Ok(())
    }

    pub fn get_args(&self) -> &QemuLaunchArgs {
        &self.args
    }

    pub fn get_mut_args(&mut self) -> &mut QemuLaunchArgs {
        &mut self.args
    }

    pub fn get_process(&self) -> &Option<QemuProcess> {
        &self.process
    }

    pub fn get_mut_process(&mut self) -> &mut Option<QemuProcess> {
        &mut self.process
    }

    pub fn is_running(&mut self) -> bool {
        match &mut self.process {
            Some(p) => p.is_running(),
            None => false,
        }
    }

    pub fn pid(&self) -> Option<u32> {
        self.process.as_ref().and_then(|p| p.pid())
    }

    pub async fn wait(&mut self) -> std::io::Result<std::process::ExitStatus> {
        match &mut self.process {
            Some(p) => p.wait().await,
            None => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "VM is not running",
            )),
        }
    }

    pub async fn terminate(&mut self) -> std::io::Result<()> {
        if let Some(p) = &mut self.process {
            p.terminate().await
        } else {
            Ok(())
        }
    }
}
