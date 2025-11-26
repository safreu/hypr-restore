use crate::{LISTENER_SERVICE, SNAPSHOT_SERVICE, destination_share, systemd};
use colored::Colorize;
use std::fs::File;
use std::process::Command;
use std::{fs, io};

pub(crate) fn execute() {
    let hypr_restore_service = r#"
[Unit]
Description=Listens to events from Hyprland to track opened Applications

After=graphical-session.target
Wants=graphical-session.target

[Service]
Type=simple

ExecStart=%h/.cargo/bin/hypr-restore listen
Environment="RUST_LOG=info"
Restart=on-failure
RestartSec=1

WorkingDirectory=%h/.local/share/hypr_restore

[Install]
WantedBy=default.target"#
        .to_string();

    let hypr_snapshot_service = r#"
[Unit]
Description=Snapshots the DB of hypr-listener

[Service]
ExecStart=%h/.cargo/bin/hypr-restore snapshot
Restart=on-failure
Type=oneshot

[Install]
WantedBy=default.target
"#
    .to_string();

    let home_dir =
        dirs::home_dir().unwrap_or_else(|| panic!("{}", "Could not get home directory".red()));

    run_step_panic(
        &format!(
            "{} {}",
            "Creating share Destination:",
            destination_share(&home_dir).display()
        ),
        || fs::create_dir_all(destination_share(&home_dir)),
    );

    run_step_panic("Creating empty classes.ignore", || {
        File::create(destination_share(&home_dir).join("classes.ignore"))
    });

    run_step_panic("Creating empty executables.path", || {
        File::create(destination_share(&home_dir).join("executables.path"))
    });

    run_step_panic(
        &format!(
            "{} {}",
            "Creating systemd(&home_dir) Destination: ",
            systemd(&home_dir).display()
        ),
        || fs::create_dir_all(systemd(&home_dir)),
    );

    run_step_panic(
        &format!(
            "{} {}",
            "Writing systemd(&home_dir) hypr-listener.service to ",
            systemd(&home_dir).display()
        ),
        || {
            fs::write(
                systemd(&home_dir).join(LISTENER_SERVICE),
                hypr_restore_service,
            )
        },
    );

    run_step_panic(
        &format!(
            "{} {}",
            "Writing systemd(&home_dir) hypr-snapshot.service to ",
            systemd(&home_dir).display()
        ),
        || {
            fs::write(
                systemd(&home_dir).join(SNAPSHOT_SERVICE),
                hypr_snapshot_service,
            )
        },
    );

    run_step_panic("Reloading systemctl", || {
        Command::new("systemctl")
            .args(["--user", "daemon-reload"])
            .status()
    });

    run_step_panic("Enabling hypr-listener.service", || {
        Command::new("systemctl")
            .args(["--user", "enable", LISTENER_SERVICE])
            .status()
    });

    run_step_panic("Starting hypr-listener.service", || {
        Command::new("systemctl")
            .args(["--user", "start", LISTENER_SERVICE])
            .status()
    });

    run_step_panic("Enabling hypr-snapshot.service", || {
        Command::new("systemctl")
            .args(["--user", "enable", SNAPSHOT_SERVICE])
            .status()
    });

    println!("{}", "Finished Installing".bright_green());
}

fn run_step_panic<F, T>(description: &str, action: F)
where
    F: FnOnce() -> io::Result<T>,
{
    println!("{}", description.blue());

    match action() {
        Ok(_) => {
            println!("{} {}", "Successful at:".green(), description.green());
        }
        Err(e) => {
            panic!("{} {} {}", "Error at:".red(), description.red(), e);
        }
    }

    println!();
}
