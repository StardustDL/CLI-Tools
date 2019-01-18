use head::*;
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
        .arg(clap::Arg::with_name("bytes")
            .short("c")
            .long("bytes")
            .takes_value(true)
            .help("Print the first NUM bytes of each file\nWith the leading '-', print all but the last NUM bytes of each file")
            .validator(valid_arg_number))
        .arg(clap::Arg::with_name("lines")
            .short("n")
            .long("lines")
            .takes_value(true)
            .conflicts_with("bytes")
            .help("Print the first NUM lines instead of the first 10\nWith the leading '-', print all but the last NUM lines of each file")
            .validator(valid_arg_number))
        .get_matches();
    let config = Config::new(&matches);

    run(config).unwrap_or_else(|err| {
        println!("Error(s): {:?}", err);
        process::exit(1);
    })
}
