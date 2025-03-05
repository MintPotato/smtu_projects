use std::{env, process};
use proto_task::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| { // &args // now we’re passing ownership of the iterator returned from env::args to Config::build directly.
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = proto_task::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}