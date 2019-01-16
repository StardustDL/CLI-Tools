use std::fs;
use std::io::{self, BufRead, BufReader, Read, Result};
use std::path;

pub struct Config<'a> {
    pub file: Option<&'a str>,
    pub number: bool,
}

impl<'a> Config<'a> {
    pub fn new(args: &'a clap::ArgMatches) -> Self {
        let file = args.value_of("FILE");
        let number = args.is_present("number");

        Config { file, number }
    }
}

pub fn run(config: Config) -> Result<()> {
    let file: Box<dyn Read> = match config.file {
        Some(name) if name != "-" => Box::new(fs::File::open(path::Path::new(&name))?),
        _ => Box::new(io::stdin()),
    };
    let reader = BufReader::new(file);
    for (number,line) in reader.lines().enumerate() {
        match config {
            Config { number: true, .. } => {
                println!("{:4}  {}", number + 1, line?);
            }
            _ => {
                println!("{}", line?);
            }
        }
    }
    Ok(())
}
