use minigrep::Config;
use std::env;

fn main() {
    let args = env::args();
    let config = Config::build(args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("failed to read the file: {}", e);
        std::process::exit(1);
    };
}
