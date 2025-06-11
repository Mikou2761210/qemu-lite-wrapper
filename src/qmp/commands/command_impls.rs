use super::QmpCommand;

//Temp
#[macro_export]
macro_rules! impl_qmp_command_constructors {
    ( $( $fn_name:ident => $cmd_str:expr ),* $(,)? ) => {
        impl QmpCommand {
            $(
                #[doc = concat!("Generates a QMP command with the \"", $cmd_str, "\" command name.")]
                pub fn $fn_name() -> Self {
                    Self::new($cmd_str)
                }
            )*
        }
    };
}

impl_qmp_command_constructors! {
    quit => "quit",
    system_powerdown => "system_powerdown",
    stop => "stop",
    cont => "cont",
    system_reset => "system_reset",
    eject => "eject",
    savevm => "savevm",
    loadvm => "loadvm",
    migrate => "migrate",
    migrate_cancel => "migrate_cancel",
    blockdev_add => "blockdev-add",
    blockdev_del => "blockdev-del",
    device_add => "device_add",
    device_del => "device_del",
    query_status => "query-status",
    query_version => "query-version",
    query_commands => "query-commands",
    query_events => "query-events",
}
