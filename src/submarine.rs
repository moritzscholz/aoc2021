use crate::file_handler::read_lines;
use std::path::Path;

pub trait Submarine {
    /// Moves the submarine
    fn change_position(&mut self, direction: SubmarineMovement);

    /// Create the product of depth & position, as answer to the challenge.
    fn position_hash(&self) -> i64;

    /// Moves the submarine based on a given list of instructions.
    /// Instructions are given as text-file with one line per instruction,
    /// formatted as described in `SubmarineMovement`.
    fn change_position_from<P>(&mut self, instruction_file: P)
    where
        P: AsRef<Path>,
    {
        let instructions = read_lines(instruction_file)
            .expect("Could not read instruction file")
            .map(|line| SubmarineMovement::from(&line.unwrap_or_default()));

        for instruction in instructions {
            self.change_position(instruction);
        }
    }
}

pub struct SimpleSubmarine {
    depth: i64,
    position_h: i64,
}

pub struct AimedSubmarine {
    depth: i64,
    position_h: i64,
    aim: i64,
}

/// Direction for the submarine to move in + distance
#[derive(PartialEq, Debug)]
pub enum SubmarineMovement {
    Forward(i64),
    Down(i64),
    Up(i64),
    Stay,
}

impl SubmarineMovement {
    /// Returns a SubmarineMovement parsed from the given `instruction` string.
    /// Example:
    ///     Given "forward 9" returns SubmarineMovement.Forward(9)
    ///
    /// Available instructions: forward, down, up
    fn from(instruction: &str) -> SubmarineMovement {
        let parts: Vec<_> = instruction.split(" ").collect();
        let direction: &str = parts[0];
        let distance: i64 = parts[1].parse().expect(&format!(
            "Could not parse direction of given instruction {}",
            instruction
        ));

        match direction {
            "forward" => SubmarineMovement::Forward(distance),
            "up" => SubmarineMovement::Up(distance),
            "down" => SubmarineMovement::Down(distance),
            &_ => SubmarineMovement::Stay,
        }
    }
}

impl SimpleSubmarine {
    /// Create a new submarine at position 0, depth 0.
    pub fn new() -> SimpleSubmarine {
        SimpleSubmarine {
            depth: 0,
            position_h: 0,
        }
    }
}

impl Submarine for SimpleSubmarine {
    fn change_position(&mut self, direction: SubmarineMovement) {
        match direction {
            SubmarineMovement::Forward(d) => self.position_h += d,
            SubmarineMovement::Up(d) => self.depth -= d,
            SubmarineMovement::Down(d) => self.depth += d,
            SubmarineMovement::Stay => (),
        }
    }

    fn position_hash(&self) -> i64 {
        self.depth * self.position_h
    }
}

impl AimedSubmarine {
    pub fn new() -> AimedSubmarine {
        AimedSubmarine {
            depth: 0,
            position_h: 0,
            aim: 0,
        }
    }
}

impl Submarine for AimedSubmarine {
    fn change_position(&mut self, direction: SubmarineMovement) {
        match direction {
            SubmarineMovement::Forward(d) => {
                self.position_h += d;
                self.depth += self.aim * d;
            }
            SubmarineMovement::Up(d) => self.aim -= d,
            SubmarineMovement::Down(d) => self.aim += d,
            SubmarineMovement::Stay => (),
        }
    }

    fn position_hash(&self) -> i64 {
        self.depth * self.position_h
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AimedSubmarine, SimpleSubmarine, Submarine, SubmarineMovement,
    };

    #[test]
    fn movement_decode() {
        assert_eq!(
            SubmarineMovement::from("forward 9"),
            SubmarineMovement::Forward(9)
        );
        assert_eq!(
            SubmarineMovement::from("up 42"),
            SubmarineMovement::Up(42)
        );
        assert_eq!(
            SubmarineMovement::from("down 1"),
            SubmarineMovement::Down(1)
        );
    }

    #[test]
    fn submarine_movement() {
        let mut submarine = SimpleSubmarine::new();
        assert_eq!(submarine.position_h, 0);
        assert_eq!(submarine.depth, 0);

        submarine.change_position(SubmarineMovement::Forward(4));
        assert_eq!(submarine.position_h, 4);
        assert_eq!(submarine.depth, 0);

        submarine.change_position(SubmarineMovement::Down(9));
        assert_eq!(submarine.position_h, 4);
        assert_eq!(submarine.depth, 9);

        submarine.change_position(SubmarineMovement::Up(3));
        assert_eq!(submarine.position_h, 4);
        assert_eq!(submarine.depth, 6);
    }

    #[test]
    fn submarine_movement_from_instruction_file() {
        let mut submarine = SimpleSubmarine::new();
        assert_eq!(submarine.position_hash(), 0);

        submarine.change_position_from("data/day2/test.txt");
        assert_eq!(submarine.position_hash(), 150);
        assert_eq!(submarine.position_h, 15);
        assert_eq!(submarine.depth, 10);
    }

    #[test]
    fn aimed_submarine_movement() {
        let mut submarine = AimedSubmarine::new();
        assert_eq!(submarine.position_hash(), 0);

        submarine.change_position_from("data/day2/test.txt");
        assert_eq!(submarine.position_hash(), 900);
        assert_eq!(submarine.position_h, 15);
        assert_eq!(submarine.depth, 60);
    }
}
