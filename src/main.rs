extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;
use minigrep::Grep;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(
        |err| {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        }
    );

    let grep = match Grep::new(&config) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("Application error: {}", e);
            process::exit(1)
        }
    };

    for line in grep {
        println!("{}", line);
    }
}
