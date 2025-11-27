use std::process::Command;
use crate::LISTENER_SERVICE;

/// Enables the hypr-listener service
pub(crate) fn execute() {
    let _ = Command::new("systemctl")
        .args(["--user", "start", LISTENER_SERVICE])
        .output();
}
