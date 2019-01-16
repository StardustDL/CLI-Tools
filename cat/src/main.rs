use cat::*;
use std::process;

#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml)
        .name(crate_name!())
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .get_matches();
    let config = Config::new(&matches);

    run(config).unwrap_or_else(|err| {
        println!("Error(s): {:?}", err);
        process::exit(1);
    })
}
