fn main() {
    if let Err(e) = bid::run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
