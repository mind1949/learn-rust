fn main() {
    if let Err(err) = uniqr::get_args().and_then(uniqr::run) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
