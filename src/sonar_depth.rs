use crate::file_handler::read_lines;
use std::path::Path;
use std::u64::MAX;

pub fn count_increases<P>(file: P) -> u64
where
    P: AsRef<Path>,
{
    let mut previous_value = MAX;
    let mut increases = 0;

    if let Ok(lines) = read_lines(file) {
        for line in lines {
            let number = line.unwrap_or_default().parse().unwrap_or(0);
            if number > previous_value {
                increases += 1;
            }

            previous_value = number;
        }
    }

    increases
}

pub fn count_increases_sliding<P>(file: P) -> u64
where
    P: AsRef<Path>,
{
    let all_numbers = read_lines(file)
        .expect("Could not read lines of file!")
        .map(|line| line.unwrap_or_default().parse().unwrap_or(0))
        .collect::<Vec<_>>();
    let all_windows = all_numbers.windows(3);

    let mut previous_window_sum: u64 = MAX;
    let mut increases: u64 = 0;

    for window in all_windows {
        let window_sum = window.iter().sum();
        if window_sum > previous_window_sum {
            increases += 1;
        }

        previous_window_sum = window_sum;
    }

    increases
}

#[cfg(test)]
mod tests {
    use super::{count_increases, count_increases_sliding};

    #[test]
    fn test_increases() {
        let increases = count_increases("data/day1/test.txt");
        assert_eq!(increases, 7);
    }

    #[test]
    fn test_increases_sliding() {
        let increases = count_increases_sliding("data/day1/test.txt");
        assert_eq!(increases, 5);
    }
}
