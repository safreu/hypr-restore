mod instance_handler_tests;

use log::{info, debug};
use std::env;
use std::io::BufReader;
use std::os::unix::net::UnixStream;
use std::thread::sleep;
use std::time::Duration;

/// Handles the hyprland instance, connects to socket and reads the events
///
/// * `reader` := The open connection to the socket
pub struct InstanceHandler {
    reader: BufReader<UnixStream>,
}

impl InstanceHandler {
    /// Constructs a new InstanceHandler
    ///
    /// # Returns
    /// Self
    pub fn new() -> Self {

        let (hypr_instance, runtime_dir) = Self::wait_for_env();

        let socket_path = format!("{}/hypr/{}/.socket2.sock", runtime_dir, hypr_instance);
        let stream = match UnixStream::connect(socket_path) {
            Ok(stream) => stream,
            Err(_) => panic!("Connect to socket failed"),
        };

        debug!("Connected to {}/hypr/{}/.socket2.sock", runtime_dir, hypr_instance);
        
        let reader = BufReader::new(stream);

        info!("Connected to Hyprland event socket, listening for events...");

        Self { reader }
    }

    /// Returns the reader
    pub fn reader(self) -> BufReader<UnixStream> {
        self.reader
    }

    /// Waits for the env variables of the HYPRLAND_INSTANCE_SIGNATURE and XDG_RUNTIME_DIR
    /// Runs MAX_RETIRES times to get the vars if getting them in time is not possible the Kernel panics
    /// 
    /// # Returns
    /// A tuple of (HYPRLAND_INSTANCE_SIGNATURE, XDG_RUNTIME_DIR)
    fn wait_for_env() -> (String, String) {
        const MAX_RETRIES: usize = 50;

        for _ in 0..MAX_RETRIES {
            let hypr_instance = env::var("HYPRLAND_INSTANCE_SIGNATURE").unwrap_or_default();
            let runtime_dir = env::var("XDG_RUNTIME_DIR").unwrap_or_default();

            if !hypr_instance.is_empty() && !runtime_dir.is_empty() {
                return (hypr_instance, runtime_dir);
            }

            sleep(Duration::from_secs(1));
        };

        let mut missing_env = Vec::new();
        if env::var("HYPRLAND_INSTANCE_SIGNATURE").unwrap_or_default().is_empty() {
            missing_env.push("HYPRLAND_INSTANCE_SIGNATURE");
        }
        if env::var("XDG_RUNTIME_DIR").unwrap_or_default().is_empty() {
            missing_env.push("XDG_RUNTIME_DIR");
        }

        panic!("Could not start because the following environment variables were missing: {:?}", missing_env);
    }
}
