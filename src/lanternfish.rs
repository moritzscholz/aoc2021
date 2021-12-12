use std::path::Path;

use crate::file_handler::read_first_line;

pub struct LanternfishColony {
    /// Buckets of fish
    /// fish[0]: number of fish with timer 0
    /// fish[1]: number of fish with timer 1
    /// etc.
    fish: Vec<u64>,
}

impl LanternfishColony {
    fn new() -> LanternfishColony {
        LanternfishColony { fish: vec![0; 9] }
    }

    pub fn from_file<P>(file: P) -> LanternfishColony
    where
        P: AsRef<Path>,
    {
        let mut colony = LanternfishColony::new();

        let ages_string = read_first_line(file);

        let ages = ages_string.split(',').map(|age| -> usize {
            age.parse().unwrap_or_else(|_| {
                panic!("Could not parse fish age to integer: {}", age)
            })
        });

        for age in ages {
            colony.fish[age] += 1;
        }

        colony
    }

    /// Number of fish in the colony.
    pub fn size(&self) -> u64 {
        self.fish.iter().sum::<u64>()
    }

    /// Advance the colony by one time-step.
    pub fn simulate_step(&mut self) {
        // Fish with timer 0 will spawn new ones in this step.
        let fish_spawning_new_ones = self.fish[0];

        // All fish reduce their timer by 1.
        self.fish.rotate_left(1);

        /*  "Each day, a 0 becomes a 6 and adds a new 8 to the end of the list"
            This is the same as: "Each day, a 0 becomes an 8 and adds a new 6 to
                the list." By rotation we moved all 0s to 8s, so we need to add
                in the 6s.
        */
        self.fish[6] += fish_spawning_new_ones;
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

    #[test]
    fn simulate_test_colony_256days() {
        let mut colony = LanternfishColony::from_file("data/day6/test.txt");

        colony.simulate_steps(256);
        assert_eq!(colony.size(), 26984457539);
    }
}
