use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(file: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(file)?;
    Ok(io::BufReader::new(file).lines())
}
