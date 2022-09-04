fn main() {
    if let Err(err) = wcr::get_args().and_then(wcr::run) {
        eprint!("{}", err);
    }
}
