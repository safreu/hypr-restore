mod disable;
mod enable;
mod install;
mod lib;
mod listener;
mod restore;
mod shared;
mod snapshot;
mod tui;
mod uninstall;
mod update;

use crate::Commands::{Disable, Enable, Install, Listen, Snapshot, Tui, Uninstall, Update};
use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(name = "hypr-restore")]
#[command(about = "Installer for hypr-restore")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Install hypr-restores full functionality")]
    Install,
    #[command(about = "Update hypr-restores")]
    Update,
    #[command(about = "Uninstall hypr-restores")]
    Uninstall,
    #[command(about = "Start the listener manually")]
    Listen,
    #[command(about = "Snapshot the DB manually")]
    Snapshot,
    #[command(about = "Open the Tui")]
    Tui,
    #[command(about = "Disable the listener")]
    Disable,
    #[command(about = "Enable the listener")]
    Enable,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        None => restore::open_window().expect("Failed to execute Restore"),
        Some(Tui) => tui::execute().expect("Failed to execute TUI"),
        Some(Listen) => listener::execute().expect("Failed to listen"),
        Some(Snapshot) => snapshot::execute().expect("Failed to execute snapshot"),
        Some(Install) => install::execute(),
        Some(Update) => update::execute(),
        Some(Uninstall) => uninstall::execute(),
        Some(Disable) => disable::execute(),
        Some(Enable) => enable::execute(),
    }
}

fn destination_share(home_dir: &Path) -> PathBuf {
    home_dir.join(".local/share/hypr_restore")
}

fn systemd(home_dir: &Path) -> PathBuf {
    home_dir.join(".config/systemd/user")
}

const LISTENER_SERVICE: &str = "hypr-listener.service";
const SNAPSHOT_SERVICE: &str = "hypr-snapshot.service";
