use std::fs;
use std::io::{self, BufRead, BufReader, Read};
use std::path;

pub fn valid_arg_number(value: String) -> Result<(), String> {
    match value.parse::<i64>() {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("Not a number")),
    }
}

pub struct Config<'a> {
    pub file: Option<clap::Values<'a>>,
    pub quiet: bool,
    pub verbose: bool,
    pub zero_terminated: bool,
    pub bytes: Option<i64>,
    pub bytes_rev: bool,
    pub lines: Option<i64>,
    pub lines_rev: bool,
}

impl<'a> Config<'a> {
    pub fn new(args: &'a clap::ArgMatches) -> Self {
        let file = args.values_of("FILE");
        let quiet = args.is_present("quiet");
        let verbose = args.is_present("verbose");
        let zero_terminated = args.is_present("zero_terminated");
        let (bytes, bytes_rev) = match args.value_of("bytes") {
            Some(bytes_raw) => (
                Some(i64::abs(bytes_raw.parse::<i64>().unwrap())),
                &bytes_raw[0..0] == "-",
            ),
            None => (None, false),
        };
        let (lines, lines_rev) = match args.value_of("lines") {
            Some(lines_raw) => (
                Some(i64::abs(lines_raw.parse::<i64>().unwrap())),
                &lines_raw[0..0] == "-",
            ),
            None => (None, false),
        };
        Config {
            file,
            quiet,
            verbose,
            zero_terminated,
            bytes,
            bytes_rev,
            lines,
            lines_rev,
        }
    }
}

pub fn run(config: Config) -> io::Result<()> {
    let files = match config.file {
        Some(items) => items.collect(),
        None => vec!["-"],
    };
    let show_name = config.verbose || !config.quiet && files.len() > 1;

    for item in files {
        let file: Box<dyn Read> = match item {
            "-" => {
                if show_name {
                    println!("{}", "==> standard input <==");
                }
                Box::new(io::stdin())
            }
            _ => {
                if show_name {
                    println!("==> {} <==", item);
                }
                Box::new(fs::File::open(path::Path::new(item))?)
            }
        };

        let reader = BufReader::new(file);

        if let Some(bytes_number) = config.bytes {
            let bytes: Vec<io::Result<u8>> = reader.bytes().collect();

            let byte_count = if config.bytes_rev {
                std::cmp::max(0, bytes.len() as i64 - bytes_number) as usize
            } else {
                std::cmp::min(bytes_number as usize, bytes.len())
            };

            let mut slice_bytes: Vec<u8> = Vec::new();

            for number in 0..byte_count {
                let content = bytes[number].as_ref().unwrap();
                slice_bytes.push(*content);
            }

            println!("{}", String::from_utf8(slice_bytes).unwrap());
        } else {
            let lines: Vec<io::Result<String>> = reader.lines().collect();

            let line_count = if let Some(lines_number) = config.lines {
                if config.lines_rev {
                    std::cmp::max(0, lines.len() as i64 - lines_number) as usize
                } else {
                    std::cmp::min(lines_number as usize, lines.len())
                }
            } else {
                std::cmp::min(10, lines.len())
            };

            for number in 0..line_count {
                let content = lines[number].as_ref().unwrap();
                if config.zero_terminated {
                    print!("{}", content);
                } else {
                    println!("{}", content);
                }
            }
        }
    }

    Ok(())
}
