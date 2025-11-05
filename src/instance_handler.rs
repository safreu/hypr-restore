use std::env;
use std::io::BufReader;
use std::os::unix::net::UnixStream;

pub struct InstanceHandler {
    reader: BufReader<UnixStream>,
}

impl InstanceHandler {
    pub fn new() -> Self {
        let hypr_instance = match env::var("HYPRLAND_INSTANCE_SIGNATURE") {
            Ok(instance) => instance,
            Err(_) => panic!("HYPRLAND_INSTANCE_SIGNATURE not defined"),
        };

        let runtime_dir = match env::var("XDG_RUNTIME_DIR") {
            Ok(dir) => dir,
            Err(_) => panic!("XDG_RUNTIME_DIR not defined"),
        };

        let socket_path = format!("{}/hypr/{}/.socket2.sock", runtime_dir, hypr_instance);

        let stream = match UnixStream::connect(socket_path) {
            Ok(stream) => stream,
            Err(_) => panic!("Connect to socket failed"),
        };

        let reader = BufReader::new(stream);

        println!("Connected to Hyprland event socket, listening for events...");

        Self { reader }
    }
    pub fn reader(self) -> BufReader<UnixStream> {
        self.reader
    }
}