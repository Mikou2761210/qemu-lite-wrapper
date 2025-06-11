# qemu-lite-wrapper

**qemu-lite-wrapper** is an asynchronous Rust library designed to simplify QEMU process management and control via QMP (QEMU Machine Protocol). It is built on top of `tokio` and supports managing multiple virtual machines as well as sending commands.

## Features

* **QMP Dispatcher**: `QmpDispatcher` dynamically routes events and reply messages to appropriate handlers.
* **Command Sender**: Use `QmpSender` to send QMP commands asynchronously.
* **QEMU Process Management**: `QemuLaunchArgs` and `QemuProcess` provide flexible command-line construction and process control.
* **Virtual Machine Management**: `VmController` and `VmManager` allow creating, terminating, and managing QMP connections for multiple VMs.

## Minimal Example

```rust
use qemu_lite_wrapper::launcher::{QemuArg, QemuLaunchArgs};
use qemu_lite_wrapper::vm::VmController;
use tokio::net::unix::{OwnedReadHalf, OwnedWriteHalf};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build QEMU binary path and launch options
    let args = QemuLaunchArgs::new("/usr/bin/qemu-system-x86_64")
        .with_flag("-nographic");

    // Create and launch a VM
    let mut vm = VmController::<OwnedReadHalf, OwnedWriteHalf>::new(args);
    vm.launch().await?;

    // Perform operations via QMP if needed

    // Terminate the VM
    vm.terminate().await?;
    Ok(())
}
```

## Build

```bash
cargo build
```

Currently, no tests are provided.

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.
