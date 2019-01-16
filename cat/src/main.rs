use cat::*;
use std::process;

#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.value_of("FILE") {
        Some(filename) => {
            run_file(std::path::Path::new(filename)).unwrap_or_else(|err| {
                println!("Error(s): {}", err);
                process::exit(1);
            });
        }
        None => {
            run_stdin().unwrap_or_else(|err| {
                println!("Error(s): {}", err);
                process::exit(1);
            });
        }
    }
}
