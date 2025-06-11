use std::process::Stdio;
use tokio::io::AsyncReadExt;
use tokio::process::Command;

use super::QemuLaunchArgs;

#[derive(Debug)]
pub struct QemuProcess {
    child: tokio::process::Child,
}

impl QemuProcess {
    pub async fn launch(args: &QemuLaunchArgs) -> std::io::Result<Self> {
        let mut cmd = Command::new(args.get_binary());

        for arg in args.get_args() {
            cmd.args(arg.to_args());
        }

        cmd.args(args.get_positionals());
        cmd.stdin(Stdio::null())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());

        let child = cmd.spawn()?;
        Ok(Self { child })
    }

    pub fn get_child(&self) -> &tokio::process::Child {
        &self.child
    }

    pub fn get_mut_child(&mut self) -> &mut tokio::process::Child {
        &mut self.child
    }

    pub async fn wait(&mut self) -> std::io::Result<std::process::ExitStatus> {
        self.child.wait().await
    }

    pub async fn terminate(&mut self) -> std::io::Result<()> {
        self.child.kill().await
    }

    pub fn is_running(&mut self) -> bool {
        self.child.try_wait().unwrap_or(None).is_none()
    }

    pub fn pid(&self) -> Option<u32> {
        self.child.id()
    }

    pub fn try_wait_exit_code(&mut self) -> Option<i32> {
        self.child.try_wait().ok().flatten().and_then(|s| s.code())
    }

    pub async fn read_stdout(&mut self) -> std::io::Result<Option<String>> {
        if let Some(stdout) = &mut self.child.stdout {
            let mut buf = String::new();
            stdout.read_to_string(&mut buf).await?;
            Ok(Some(buf))
        } else {
            Ok(None)
        }
    }

    pub async fn read_stderr(&mut self) -> std::io::Result<Option<String>> {
        if let Some(stderr) = &mut self.child.stderr {
            let mut buf = String::new();
            stderr.read_to_string(&mut buf).await?;
            Ok(Some(buf))
        } else {
            Ok(None)
        }
    }
}
