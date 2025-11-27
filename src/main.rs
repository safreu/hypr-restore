fn main() {
    if let Err(e) = hypr_restore::run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
