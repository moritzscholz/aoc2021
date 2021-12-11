use std::fs::{read_to_string, File};
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(file: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(file)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_all<P>(file: P) -> std::io::Result<String>
where
    P: AsRef<Path>,
{
    read_to_string(file)
}
