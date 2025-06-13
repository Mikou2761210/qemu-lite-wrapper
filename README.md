# qemu-lite-wrapper

**qemu-lite-wrapper** is an asynchronous Rust library designed to simplify QEMU process management and control via QMP (QEMU Machine Protocol). It is built on top of `tokio` and supports managing multiple virtual machines as well as sending commands.

## Features

* **QMP Dispatcher**: `QmpDispatcher` dynamically routes events and reply messages to appropriate handlers.
* **Command Sender**: Use `QmpSender` to send QMP commands asynchronously.
* **QEMU Process Management**: `QemuLaunchArgs` and `QemuProcess` provide flexible command-line construction and process control.
* **Virtual Machine Management**: `VmController` and `VmManager` allow creating, terminating, and managing QMP connections for multiple VMs.
* **Example VM Module**: The structs under `src/vm` are lightweight samples created for demonstration.

## Minimal Example

```rust
use qemu_lite_wrapper::launcher::{QemuArg, QemuLaunchArgs};
use qemu_lite_wrapper::vm::VmController;
use tokio::net::unix::{OwnedReadHalf, OwnedWriteHalf};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build QEMU binary path and launch options
    let args = QemuLaunchArgs::new("/usr/bin/qemu-system-x86_64")
        .with_flag("-nographic") // Same as `.with_arg(QemuArg::from_flag("-nographic"))`
        .with_key_value("cpu", "host") // Same as `.with_arg(QemuArg::from_key_value("cpu", "host"))`
        .with_list("device", ["virtio-net", "netdev=net0"]); // Same as `.with_arg(QemuArg::from_list("device", [...]))`

    // Create and launch a VM
    let mut vm = VmController::<OwnedReadHalf, OwnedWriteHalf>::new(args);
    vm.launch().await?;

    // Perform operations via QMP if needed

    // Terminate the VM
    vm.terminate().await?;
    Ok(())
}
```
### QMP Connection and Command Sending Example

```rust
use qemu_lite_wrapper::launcher::QemuLaunchArgs;
use qemu_lite_wrapper::qmp::commands::{QmpCommand, QmpSender};
use qemu_lite_wrapper::qmp::dispatcher::QmpDispatcher;
use qemu_lite_wrapper::qmp::streams::QmpMessageStream;
use qemu_lite_wrapper::qmp::types::QmpId;
use qemu_lite_wrapper::vm::VmController;
use tokio::net::UnixStream;
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = QemuLaunchArgs::new("/usr/bin/qemu-system-x86_64")
        .with_key_value("-qmp", "unix:/tmp/qmp.sock,server,nowait");

    // Create and launch the VM
    let mut vm = VmController::new(args);
    vm.launch().await?;

    // Connect to QMP socket
    let stream = UnixStream::connect("/tmp/qmp.sock").await?;
    let (r, w) = stream.into_split();
    vm.set_sender(Some(QmpSender::new(w)));
    vm.set_message_stream(Some(QmpMessageStream::new(r, CancellationToken::new())));

    // Register a reply handler
    let mut dispatcher = QmpDispatcher::new();
    dispatcher.register_reply_handler(QmpId::Num(1), |rep| {
        println!("reply: {:?}", rep);
    });

    // Send a QMP command
    vm.send_command(&QmpCommand::query_status().with_id(QmpId::Num(1))).await?;

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
