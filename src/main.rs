fn main() {
    println!("{:?}", count_increases("input.txt"));
}

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::u64::MAX;

fn count_increases<P>(file: P) -> u64
where
    P: AsRef<Path>,
{
    let mut previous_value = MAX;
    let mut increases = 0;

    if let Ok(lines) = read_lines(file) {
        for line in lines {
            let number = line.unwrap_or(String::new()).parse().unwrap_or(0);
            if number > previous_value {
                increases += 1;
            }

            previous_value = number;
        }
    }

    increases
}

fn read_lines<P>(file: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(file)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use crate::count_increases;

    #[test]
    fn test_increases() {
        let increases = count_increases("test.txt");
        assert_eq!(increases, 7);
    }
}
