use std::collections::HashMap;
use std::path::Path;

use crate::file_handler::read_lines;
pub struct Polymerizer {
    chain: String,
    instructions: HashMap<(char, char), char>,
}

impl Polymerizer {
    pub fn from_file<P>(file: P) -> Polymerizer
    where
        P: AsRef<Path>,
    {
        let mut lines = read_lines(file).expect("Could not read input file.");

        // First line is the chain
        let chain = lines
            .next()
            .expect("Could not read chain from file.")
            .expect("Could not read chain from file.");

        // Next line is empty
        lines.next();

        // Following lines contain instructions
        let instructions: HashMap<(char, char), char> = lines
            .map(|line| {
                let line_string = line.expect("Could not read line");
                let parts: Vec<&str> = line_string.split(" -> ").collect();
                let ab = parts[0].chars().collect::<Vec<char>>();
                let x = parts[1]
                    .chars()
                    .next()
                    .expect("Could not parse insertion element.");

                ((ab[0], ab[1]), x)
            })
            .collect();

        Polymerizer {
            chain,
            instructions,
        }
    }

    // Processes the next instruction
    pub fn step(&mut self) {
        let mut new_chain = self.chain.clone();

        for (idx, pair) in self
            .chain
            .chars()
            .collect::<Vec<char>>()
            .windows(2)
            .enumerate()
        {
            print!("{}", idx);
            let a = pair[0];
            let b = pair[1];

            if let Some(replacement) = self.instructions.get(&(a, b)) {
                new_chain.insert(idx + 1, *replacement);
            }
        }

        self.chain = new_chain;
    }

    /// Calculates puzzle solution
    /// qty of most common element - qty of least common element
    pub fn solution(&self) -> i32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_steps() {
        let mut polymerizer = Polymerizer::from_file("data/day14/test.txt");
        assert_eq!(polymerizer.chain, "NNCB");
        polymerizer.step();
        assert_eq!(polymerizer.chain, "NBCCNBBBCBHCB");
        polymerizer.step();
        assert_eq!(polymerizer.chain, "NBBBCNCCNBBNBNBBCHBHHBCHB");
        polymerizer.step();
        assert_eq!(
            polymerizer.chain,
            "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB"
        );
    }
}
