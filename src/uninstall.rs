use crate::{LISTENER_SERVICE, SNAPSHOT_SERVICE, destination_share, systemd};
use colored::Colorize;
use std::fs;
use std::process::Command;

pub(crate) fn execute() {
    let home_dir =
        dirs::home_dir().unwrap_or_else(|| panic!("{}", "Could not find home directory".red()));

    run_step_eprintln(
        &format!(
            "{} {}",
            "Removing share:",
            destination_share(&home_dir).display()
        ),
        || fs::remove_dir_all(destination_share(&home_dir)).map_err(|e| e.to_string()),
    );

    run_step_eprintln("Stopping hypr-listener.service", || {
        Command::new("systemctl")
            .args(["--user", "stop", LISTENER_SERVICE])
            .status()
            .map_err(|e| e.to_string())
    });

    run_step_eprintln("Disabling hypr-listener.service", || {
        Command::new("systemctl")
            .args(["--user", "disable", LISTENER_SERVICE])
            .status()
            .map_err(|e| e.to_string())
    });

    run_step_eprintln("Stopping hypr-snapshot.service", || {
        Command::new("systemctl")
            .args(["--user", "stop", SNAPSHOT_SERVICE])
            .status()
            .map_err(|e| e.to_string())
    });

    run_step_eprintln("Disabling hypr-snapshot.service", || {
        Command::new("systemctl")
            .args(["--user", "disable", SNAPSHOT_SERVICE])
            .status()
            .map_err(|e| e.to_string())
    });

    run_step_eprintln("Reloading systemctl", || {
        Command::new("systemctl")
            .args(["--user", "daemon-reload"])
            .status()
            .map_err(|e| e.to_string())
    });

    run_step_eprintln("Removing hypr-listener.service", || {
        fs::remove_file(systemd(&home_dir).join(LISTENER_SERVICE)).map_err(|e| e.to_string())
    });

    run_step_eprintln("Removing hypr-snapshot.service", || {
        fs::remove_file(systemd(&home_dir).join(SNAPSHOT_SERVICE)).map_err(|e| e.to_string())
    });

    println!("{}", "Finished Uninstalling".bright_green());
}

fn run_step_eprintln<F, T>(description: &str, action: F)
where
    F: FnOnce() -> Result<T, String>,
{
    println!("{}", description.blue());

    match action() {
        Ok(_) => {
            println!("{} {}", "Successful at:".green(), description.green());
        }
        Err(e) => {
            eprintln!("{} {} {}", "Error at:".red(), description.red(), e);
        }
    }

    println!();
}

