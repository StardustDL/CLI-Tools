use std::fs;
use std::io::{self, BufRead, BufReader, Read, Result};
use std::path;

pub struct Config<'a> {
    pub file: Option<clap::Values<'a>>,
    pub number: bool,
    pub number_nonblank: bool,
    pub show_ends: bool,
    pub show_tabs: bool,
    pub show_nonprinting: bool,
    pub squeeze_blank: bool,
}

impl<'a> Config<'a> {
    pub fn new(args: &'a clap::ArgMatches) -> Self {
        let file = args.values_of("FILE");
        let number = args.is_present("number");
        let number_nonblank = args.is_present("number-nonblank");
        let show_all = args.is_present("show-all");
        let show_ve = args.is_present("show-vE") | show_all;
        let show_ends = args.is_present("show-ends") | show_ve;
        let show_vt = args.is_present("show-vT") | show_all;
        let show_tabs = args.is_present("show-tabs") | show_vt;
        let show_nonprinting = args.is_present("show-nonprinting") | show_ve | show_vt;
        let squeeze_blank = args.is_present("squeeze-blank");
        Config {
            file,
            number,
            number_nonblank,
            show_ends,
            show_tabs,
            show_nonprinting,
            squeeze_blank,
        }
    }
}

pub fn run(config: Config) -> Result<()> {
    let files = match config.file {
        Some(items) => items.collect(),
        None => vec!["-"],
    };
    for item in files {
        let file: Box<dyn Read> = match item {
            "-" => Box::new(io::stdin()),
            _ => Box::new(fs::File::open(path::Path::new(item))?),
        };

        let reader = BufReader::new(file);
        let mut number_nonblank = 0;
        let mut last_empty = false;
        for (number, line_raw) in reader.lines().enumerate() {
            let mut line = line_raw?;
            let is_empty = line.is_empty();
            if is_empty && last_empty && config.squeeze_blank {
                continue;
            }
            if config.show_tabs {
                line = line.replace("\t", "^I");
            }
            if config.show_nonprinting {
                line = line.replace("\r", "^M");
            }
            if config.show_ends {
                line.push_str("$");
            }
            if config.number_nonblank {
                if is_empty {
                    println!("{:4}  {}", "", line);
                } else {
                    number_nonblank += 1;
                    println!("{:4}  {}", number_nonblank, line);
                }
            } else if config.number {
                println!("{:4}  {}", number + 1, line);
            } else {
                println!("{}", line);
            }
            last_empty = is_empty;
        }
    }

    Ok(())
}
