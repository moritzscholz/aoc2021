mod fish;
use fish::Lanternfish;
use std::path::Path;

use crate::file_handler::read_lines;

pub struct LanternfishColony {
    fish: Vec<Lanternfish>,
}

impl LanternfishColony {
    pub fn from_file<P>(file: P) -> LanternfishColony
    where
        P: AsRef<Path>,
    {
        let fish: Vec<Lanternfish> = read_lines(file)
            .expect("Could not read file containing age of the fishes.")
            .next()
            .expect("Could not read first line of the given file.")
            .expect("Could not turn the first line of the file into a string.")
            .split(',')
            .map(|age| {
                age.parse().unwrap_or_else(|_| {
                    panic!("Could not parse fish age to integer: {}", age)
                })
            })
            .map(Lanternfish::new)
            .collect();

        LanternfishColony { fish }
    }

    /// Number of fish in the colony.
    pub fn size(&self) -> usize {
        self.fish.len()
    }

    /// Advance the colony by one time-step.
    pub fn simulate_step(&mut self) {
        let mut new_fish: Vec<Lanternfish> = self
            .fish
            .iter_mut()
            .filter_map(|fish| fish.grow_up())
            .collect();

        self.fish.append(&mut new_fish);
    }

    /// Advance the colony by n time-steps.
    pub fn simulate_steps(&mut self, steps: u32) {
        for _ in 1..steps + 1 {
            self.simulate_step()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simulate_test_colony() {
        let mut colony = LanternfishColony::from_file("data/day6/test.txt");

        colony.simulate_steps(18);
        assert_eq!(colony.size(), 26);

        colony.simulate_steps(80 - 18);
        assert_eq!(colony.size(), 5934);
    }
}
