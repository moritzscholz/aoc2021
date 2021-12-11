use std::path::Path;

use crate::file_handler::read_lines;

/// Tracks number of zeroes and ones over the column of a list of binary numbers.

#[derive(Clone)]
struct Column {
    zeroes: u32,
    ones: u32,
}

struct Columns {
    columns: Vec<Column>,
}

pub struct DiagnosticsResult {
    gamma_rate: u32,
    epsilon_rate: u32,
}

impl Column {
    fn new() -> Column {
        Column { zeroes: 0, ones: 0 }
    }

    /// Updates count of zeroes or ones based on the given digit as a string.
    fn update(&mut self, digit: char) {
        match digit {
            '0' => self.zeroes += 1,
            '1' => self.ones += 1,
            _ => (),
        }
    }

    fn most_common_digit(&self) -> &str {
        if self.zeroes > self.ones {
            "0"
        } else {
            "1"
        }
    }
}

impl Columns {
    /// Creates a new array of columns according to the length of the binary
    /// number in `line`.
    fn from(line: &str) -> Columns {
        Columns {
            columns: vec![Column::new(); line.len()],
        }
    }

    pub fn update(&mut self, line: &str) {
        assert!(self.columns.len() <= line.len(), "Line too short!");
        if self.columns.len() < line.len() {
            println!("Warning: Line is longer than available columns.")
        }

        for (idx, column) in self.columns.iter_mut().enumerate() {
            let digit = line.chars().collect::<Vec<_>>()[idx];
            column.update(digit);
        }
    }

    pub fn most_common_digits(&self) -> String {
        self.columns
            .iter()
            .map(|column| column.most_common_digit())
            .collect::<String>()
    }
}

impl DiagnosticsResult {
    fn new(gamma_rate: u32, epsilon_rate: u32) -> DiagnosticsResult {
        DiagnosticsResult {
            gamma_rate,
            epsilon_rate,
        }
    }

    pub fn diagnose<P>(file: P) -> DiagnosticsResult
    where
        P: AsRef<Path>,
    {
        let mut lines = read_lines(file).expect("Could not read input file.");

        // Initialize digit-counters for the columns with first line & process that line
        let header = lines
            .next()
            .expect("File does not have a header (maybe empty?).")
            .expect("Could not read header line.");
        let mut columns = Columns::from(&header);
        columns.update(&header);

        // Process the rest of the file
        for line in lines {
            columns.update(&line.unwrap_or_default())
        }

        // Build binary number out of most common digits
        let gamma_rate_binary = columns.most_common_digits();
        let gamma_rate: u32 = u32::from_str_radix(&gamma_rate_binary, 2)
            .expect("Could not parse binary gamma rate to decimal.");
        let epsilon_rate: u32 =
            u32::from_str_radix(&flip_bits(&gamma_rate_binary), 2)
                .expect("Could not parse binary gamma rate to decimal.");

        DiagnosticsResult::new(gamma_rate, epsilon_rate)
    }

    pub fn power_consumption(&self) -> u32 {
        self.gamma_rate * self.epsilon_rate
    }
}

/// Flips bits in given `binary_string`.
/// Example:
///     Given 100110 returns 011001
/// TODO: I think there's a better way to do this, using `!`, but not sure how.
fn flip_bits(binary_string: &str) -> String {
    binary_string
        .chars()
        .map(|char| match char {
            '0' => '1',
            '1' => '0',
            _ => panic!("Tried to bit-flip binary string but encountered character which is neither 0 nor 1.")
        })
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_diagnosis() {
        let result = DiagnosticsResult::diagnose("data/day3/test.txt");
        assert_eq!(result.gamma_rate, 22);
        assert_eq!(result.epsilon_rate, 9);
        assert_eq!(result.power_consumption(), 198);
    }
}
