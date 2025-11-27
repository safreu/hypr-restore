mod instance_handler_tests;

use log::info;
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
        let mut counter = 0;
        let mut hypr_instance: String = String::from("");
        let mut runtime_dir: String = String::from("");
        while (hypr_instance.is_empty() && runtime_dir.is_empty()) || counter >= 50 {
            hypr_instance = env::var("HYPRLAND_INSTANCE_SIGNATURE").unwrap_or_default();

            runtime_dir = env::var("XDG_RUNTIME_DIR").unwrap_or_default();

            counter += 1;
            sleep(Duration::from_secs(1));
        }

        if hypr_instance.is_empty() {
            panic!("HYPRLAND_INSTANCE_SIGNATURE not defined")
        }
        if runtime_dir.is_empty() {
            panic!("XDG_RUNTIME_DIR not defined")
        }

        let socket_path = format!("{}/hypr/{}/.socket2.sock", runtime_dir, hypr_instance);

        let stream = match UnixStream::connect(socket_path) {
            Ok(stream) => stream,
            Err(_) => panic!("Connect to socket failed"),
        };

        let reader = BufReader::new(stream);

        info!("Connected to Hyprland event socket, listening for events...");

        Self { reader }
    }

    /// Returns the reader
    pub fn reader(self) -> BufReader<UnixStream> {
        self.reader
    }
}
