fn main() {
    if let Err(err) = findr::get_args().and_then(findr::run) {
        eprint!("{}", err);
        std::process::exit(1);
    }
}
