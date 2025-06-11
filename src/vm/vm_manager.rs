use std::collections::HashMap;
use tokio::io::{AsyncRead, AsyncWrite};

use super::VmController;
use crate::launcher::QemuLaunchArgs;

type VmCtrl = VmController<
    Box<dyn AsyncRead + Unpin + Send + 'static>,
    Box<dyn AsyncWrite + Unpin + Send + 'static>,
>;

pub struct VmManager {
    vms: HashMap<String, VmCtrl>,
}

impl VmManager {
    pub fn new() -> Self {
        Self {
            vms: HashMap::new(),
        }
    }

    pub fn create_vm(&mut self, name: impl Into<String>, args: QemuLaunchArgs) {
        self.vms.insert(name.into(), VmController::new(args));
    }

    pub fn get_vm(&mut self, name: &str) -> Option<&mut VmCtrl> {
        self.vms.get_mut(name)
    }

    pub fn get_all_vms(&mut self) -> impl Iterator<Item = (&String, &mut VmCtrl)> {
        self.vms.iter_mut()
    }

    pub fn get_all_vm_names(&self) -> Vec<String> {
        self.vms.keys().cloned().collect()
    }

    pub fn remove_vm(&mut self, name: &str) -> Option<VmCtrl> {
        self.vms.remove(name)
    }

    pub async fn shutdown_all(&mut self) -> std::io::Result<()> {
        for (_, vm) in &mut self.vms {
            let _ = vm.terminate().await;
        }
        Ok(())
    }
}
