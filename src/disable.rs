use std::process::Command;

pub(crate) fn execute() {
    let _ = Command::new("systemctl")
        .args(["--user", "stop", "hypr-listener"])
        .output();
}
