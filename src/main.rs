use std::env;
use std::process;

use greprs::Config;

fn main() {
    // --snip--
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = greprs::run(config) {
        // --snip--
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
