use std::{collections::VecDeque, path::Path};

use crate::file_handler::read_lines;

#[derive(Debug)]
enum FoldingInstruction {
    FoldUp(usize),
    FoldLeft(usize),
}

impl From<String> for FoldingInstruction {
    fn from(s: String) -> Self {
        let number: usize = s
            .split('=')
            .nth(1)
            .expect("Error getting number of folding instruction")
            .parse()
            .expect(
                "Error converting number of folding instruction to integer.",
            );

        if s.starts_with("fold along y=") {
            FoldingInstruction::FoldUp(number)
        } else if s.starts_with("fold along x=") {
            FoldingInstruction::FoldLeft(number)
        } else {
            panic!("Encountered unknown folding instruction {}", s);
        }
    }
}

pub struct TransparentPaper {
    // True, if position x, y is marked
    markings: Vec<Vec<bool>>,
    instructions: VecDeque<FoldingInstruction>,
}

impl TransparentPaper {
    pub fn from_file<P>(file: P) -> TransparentPaper
    where
        P: AsRef<Path>,
    {
        let mut markings_list: Vec<(usize, usize)> = vec![];
        let mut dimension_x: usize = 0;
        let mut dimension_y: usize = 0;
        let mut instructions: VecDeque<FoldingInstruction> = VecDeque::new();

        let lines = read_lines(file).expect("Error reading file");
        for line in lines {
            let line = line.expect("Could not parse line");

            if line.is_empty() {
                continue;
            }

            if line.starts_with("fold") {
                instructions.push_back(FoldingInstruction::from(line));
            } else {
                let coordinate = line.split(',').collect::<Vec<&str>>();

                let x: usize = coordinate[0]
                    .parse()
                    .expect("Error parsing x-coordinate.");
                let y: usize = coordinate[1]
                    .parse()
                    .expect("Error parsing y-coordinate.");

                dimension_x = dimension_x.max(x);
                dimension_y = dimension_y.max(y);

                markings_list.push((x, y));
            }
        }

        let mut markings: Vec<Vec<bool>> =
            vec![vec![false; dimension_y + 1]; dimension_x + 1];
        for (x, y) in markings_list {
            markings[x][y] = true;
        }

        TransparentPaper {
            markings,
            instructions,
        }
    }

    /// Folds the paper consuming one instruction.
    /// Returns `true` if folded, returns `false` if no instructionw as left.
    pub fn fold(&mut self) -> bool {
        if let Some(instruction) = self.instructions.pop_front() {
            println!(
                "Paper dimension is ({}, {}). Folding with instruction {:?}.",
                self.width(),
                self.height(),
                instruction
            );
            self.fold_with_instruction(instruction);
            true
        } else {
            false
        }
    }

    fn fold_with_instruction(&mut self, instruction: FoldingInstruction) {
        match instruction {
            FoldingInstruction::FoldUp(y) => self.fold_up(y),
            FoldingInstruction::FoldLeft(x) => self.fold_left(x),
        }
    }

    fn fold_up(&mut self, y: usize) {
        let mut new_markings = vec![vec![false; y]; self.width()];

        for (x, column) in self.markings.iter().enumerate() {
            // Make a new, shorter column with all the items above & excluding the fold-line
            let mut new_column: Vec<bool> = column.clone()[0..y].to_vec();

            // Fold items below the fold-line upwards.
            for (idx, marked) in column.iter().skip(y + 1).enumerate() {
                let target_idx = 2 * y - (idx + y + 1);
                new_column[target_idx] = *marked || new_column[target_idx];
            }

            new_markings[x] = new_column;
        }

        self.markings = new_markings;
    }

    fn fold_left(&mut self, x: usize) {
        let start = 2 * x - (self.width() - 1);
        let mut new_markings = vec![vec![false; self.height()]; x];

        for (idx, column) in self.markings.iter().enumerate() {
            if idx >= x {
                continue;
            }

            let new_column: Vec<bool>;

            if idx < start {
                new_column = column.to_vec();
            } else {
                let folded_column = self.markings[2 * x - idx].to_vec();
                new_column = column
                    .iter()
                    .zip(folded_column.iter())
                    .map(|(&a, &b)| a || b)
                    .collect();
            }

            new_markings[idx] = new_column;
        }

        self.markings = new_markings;
    }

    fn height(&self) -> usize {
        self.markings[0].len()
    }

    fn width(&self) -> usize {
        self.markings.len()
    }

    /// Returns number of marked fields
    pub fn num_marked(&self) -> usize {
        self.markings.iter().flatten().filter(|item| **item).count()
    }

    /// Prints the paper
    pub fn dump(&self) {
        // This is done very badle... :/
        let mut transposed: Vec<Vec<bool>> =
            vec![vec![false; self.width()]; self.height()];

        for (x, column) in self.markings.iter().enumerate() {
            for (y, &marked) in column.iter().enumerate() {
                transposed[y][x] = marked;
            }
        }

        for line in transposed {
            for marked in line {
                if marked {
                    print!("#")
                } else {
                    print!(".")
                }
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_fold() {
        let mut paper = TransparentPaper::from_file("data/day13/test.txt");
        paper.dump();
        paper.fold();
        paper.dump();
        assert_eq!(paper.num_marked(), 17);
    }

    #[test]
    fn test_two_folds() {
        let mut paper = TransparentPaper::from_file("data/day13/test.txt");
        paper.dump();
        paper.fold();
        paper.dump();
        paper.fold();
        paper.dump();
        assert_eq!(paper.num_marked(), 16);
    }
}
