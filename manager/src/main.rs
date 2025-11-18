mod install;
mod update;
mod uninstall;

use std::path::{PathBuf};
use clap::{Parser, Subcommand};
use crate::Commands::{Tui, Snapshot, Listen, Install, Uninstall, Update};

#[derive(Parser)]
#[command(name = "hypr-restore")]
#[command(about = "Installer for Hypr-restore")]
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
    #[command(about = "open the tui")]
    Tui,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        None => restore::open_window().expect("Failed to execute Restore"),
        Some(Tui) => todo!(),
        Some(Listen) => listener::execute().expect("Failed to listen"),
        Some(Snapshot) => snapshot::execute().expect("Failed to execute snapshot"),
        Some(Install) => install::execute(),
        Some(Update) => update::execute(),
        Some(Uninstall) => uninstall::execute(),
    }
}

fn destination_share(home_dir: &PathBuf) -> PathBuf {
    home_dir.join(".local/share/hypr_restore")
}

fn systemd(home_dir: &PathBuf) -> PathBuf {
    home_dir.join(".config/systemd/user")
}

const LISTENER_SERVICE: &str = "hypr-listener.service";
const SNAPSHOT_SERVICE: &str = "hypr-snapshot.service";