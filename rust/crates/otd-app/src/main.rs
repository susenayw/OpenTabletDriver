fn main() {
    if let Err(error) = otd_app::run() {
        eprintln!("Rust Tablet: {error}");
        std::process::exit(1);
    }
}
