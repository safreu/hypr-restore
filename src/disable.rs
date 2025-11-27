use std::process::Command;
use crate::LISTENER_SERVICE;

/// Disables the hypr-listener service
pub(crate) fn execute() {
    let _ = Command::new("systemctl")
        .args(["--user", "stop", LISTENER_SERVICE])
        .output();
}
