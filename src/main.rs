use std::env;
use std::process;

use minigrep_learn::Config;

fn main() {
    let config_result = Config::build(env::args());

    let config = match config_result {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Problem parsing arguments: {err}");
            process::exit(1);
        }
    };

    if let Err(err) = minigrep_learn::run(config) {
        eprintln!("Application error: {err}");
        process::exit(1);
    }

}
