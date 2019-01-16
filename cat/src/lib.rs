use std::fs;
use std::io::{self, BufRead, BufReader, Read, Result};
use std::path;

pub struct Config<'a> {
    pub file: Option<&'a str>,
    pub number: bool,
    pub number_nonblank: bool,
}

impl<'a> Config<'a> {
    pub fn new(args: &'a clap::ArgMatches) -> Self {
        let file = args.value_of("FILE");
        let number = args.is_present("number");
        let number_nonblank = args.is_present("number-nonblank");
        Config {
            file,
            number,
            number_nonblank,
        }
    }
}

pub fn run(config: Config) -> Result<()> {
    let file: Box<dyn Read> = match config.file {
        Some(name) if name != "-" => Box::new(fs::File::open(path::Path::new(&name))?),
        _ => Box::new(io::stdin()),
    };

    let reader = BufReader::new(file);
    let mut number_nonblank = 0;
    for (number, line_raw) in reader.lines().enumerate() {
        let line = line_raw?;
        match config {
            Config {
                number_nonblank: true,
                ..
            } => {
                if line.is_empty() {
                    println!("{:4}  {}", "", line);
                } else {
                    number_nonblank += 1;
                    println!("{:4}  {}", number_nonblank, line);
                }
            }
            Config { number: true, .. } => {
                println!("{:4}  {}", number + 1, line);
            }
            _ => {
                println!("{}", line);
            }
        }
    }

    Ok(())
}
