use crate::file_handler::read_lines;
use std::path::Path;

type Height = u32;
pub struct CaveFloorScanner {
    height_map: Vec<Vec<Height>>,
}

impl CaveFloorScanner {
    /// Creates a new cave floor scanner, loading the height-map at path `file`.
    pub fn from_file<P>(file: P) -> CaveFloorScanner
    where
        P: AsRef<Path>,
    {
        let height_map: Vec<Vec<Height>> = read_lines(file)
            .expect("Could not read heightmap-file")
            .map(|line| {
                line.expect("Could not read line in heightmap")
                    .chars()
                    .map(|character| {
                        character.to_digit(10).unwrap_or_else(|| {
                            panic!(
                                "Could not parse character {} to integer.",
                                character
                            )
                        })
                    })
                    .collect()
            })
            .collect();

        CaveFloorScanner { height_map }
    }

    /// Returns height of point at coordinate (row, col)
    fn height(&self, row: usize, col: usize) -> Height {
        self.height_map[row][col]
    }

    /// Gives all heights surrounding the given coordinate (row, col)
    fn surrounding_heights(&self, row: usize, col: usize) -> Vec<Height> {
        let mut heights: Vec<Height> = vec![];

        // Up
        if row > 0 {
            heights.push(self.height(row - 1, col));
        }

        // Down
        if row < self.height_map.len() - 1 {
            heights.push(self.height(row + 1, col));
        }

        // Left
        if col > 0 {
            heights.push(self.height(row, col - 1));
        }

        // Right
        if col < self.height_map[row].len() - 1 {
            heights.push(self.height(row, col + 1));
        }

        heights
    }

    /// Finds all lowest locations (== all surrounding locations are higher)
    /// and returns their heights.
    fn find_heights_of_low_points(&self) -> Vec<Height> {
        // Stores (row, col)-coordinates of all low-points
        let mut low_points: Vec<(usize, usize)> = vec![];

        for row in 0..self.height_map.len() {
            for col in 0..self.height_map[row].len() {
                if self.height(row, col)
                    < self
                        .surrounding_heights(row, col)
                        .into_iter()
                        .min()
                        .unwrap_or(u32::MAX)
                {
                    low_points.push((row, col));
                }
            }
        }

        low_points
            .into_iter()
            .map(|(row, col)| self.height(row, col))
            .collect()
    }

    /// Returns the sum of the risk-levels of all low-points in the cave.
    pub fn cave_risk(&self) -> Height {
        self.find_heights_of_low_points()
            .iter()
            .map(|height| height + 1)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cave_risk() {
        let scanner = CaveFloorScanner::from_file("data/day9/test.txt");
        assert_eq!(scanner.cave_risk(), 15);
    }
}
