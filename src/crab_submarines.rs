use std::path::Path;

use crate::file_handler::read_first_line;

pub struct CrabSubmarineFleet {
    /// crabs[x] indicates number of crabs at position x.
    crabs: Vec<i32>,
}

impl CrabSubmarineFleet {
    pub fn from_file<P>(file: P) -> CrabSubmarineFleet
    where
        P: AsRef<Path>,
    {
        let crab_positions_string = read_first_line(file);
        let crab_positions: Vec<usize> = crab_positions_string
            .split(',')
            .map(|position| {
                position.parse::<usize>().unwrap_or_else(|_| {
                    panic!("Could not parse crab position {}!", position)
                })
            })
            .collect();

        let mut crabs = vec![0; *crab_positions.iter().max().expect("Could not determine max crab position. Maybe the list of positions is empty?")+1];
        for position in crab_positions {
            crabs[position] += 1;
        }

        CrabSubmarineFleet { crabs }
    }

    fn fuel_cost(&self, destination: usize) -> i32 {
        self.crabs
            .iter()
            .enumerate()
            .map(|(position, n_crabs)| {
                (destination as i32 - position as i32).abs() * n_crabs
            })
            .sum()
    }

    fn min_position(&self) -> usize {
        self.crabs
            .iter()
            .position(|n_crabs| n_crabs > &0)
            .unwrap_or(0)
    }

    fn max_position(&self) -> usize {
        self.crabs.len() - 1
    }

    /// Returns the position to move the crab submarine fleet to that requires
    /// the least fuel to move to.
    pub fn ideal_position_and_fuel(&self) -> (usize, i32) {
        let search_range = self.min_position()..self.max_position();

        let mut min_fuel_cost = self.fuel_cost(self.max_position());
        let mut ideal_destination = 0;
        for destination in search_range {
            let fuel_cost = self.fuel_cost(destination);
            if fuel_cost < min_fuel_cost {
                min_fuel_cost = fuel_cost;
                ideal_destination = destination
            }
        }

        (ideal_destination, min_fuel_cost)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuel_costs() {
        let fleet = CrabSubmarineFleet::from_file("data/day7/test.txt");
        assert_eq!(fleet.fuel_cost(2), 37);
        assert_eq!(fleet.fuel_cost(1), 41);
        assert_eq!(fleet.fuel_cost(3), 39);
        assert_eq!(fleet.fuel_cost(10), 71);
    }

    #[test]
    fn ideal_position() {
        let fleet = CrabSubmarineFleet::from_file("data/day7/test.txt");
        assert_eq!(fleet.ideal_position_and_fuel(), (2, 37));
    }
}
