use std::fs;
use std::io;
use std::path;

pub fn run_file(filename: &path::Path) -> io::Result<()> {
    run_readable(fs::File::open(filename)?)
}

pub fn run_stdin() -> io::Result<()> {
    run_readable(io::stdin())
}

fn run_readable<T: io::Read>(mut item: T) -> io::Result<()> {
    let mut contents = String::new();
    item.read_to_string(&mut contents)?;
    println!("{}", contents);
    Ok(())
}
