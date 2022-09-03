fn main() {
    if let Err(e) = catr::get_args().and_then(catr::run) {
        eprint!("{}", e);
        std::process::exit(1);
    }
}
