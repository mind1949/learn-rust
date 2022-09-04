fn main() {
    if let Err(e) = headr::get_args().and_then(headr::run) {
        eprint!("{}", e);
        std::process::exit(1);
    }
}
