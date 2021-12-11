struct Submarine {
    depth: i64,
    position: i64,
}

/// Direction for the submarine to move in + distance
#[derive(PartialEq, Debug)]
enum SubmarineMovement {
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

impl Submarine {
    /// Create a new submarine at position 0, depth 0.
    fn new() -> Submarine {
        Submarine {
            depth: 0,
            position: 0,
        }
    }

    /// Moves the submarine
    fn change_position(&mut self, direction: SubmarineMovement) {
        match direction {
            SubmarineMovement::Forward(d) => self.position += d,
            SubmarineMovement::Up(d) => self.depth -= d,
            SubmarineMovement::Down(d) => self.depth += d,
            SubmarineMovement::Stay => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Submarine, SubmarineMovement};

    #[test]
    fn test_movement_decode() {
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
    fn test_submarine_movement() {
        let mut submarine = Submarine::new();
        assert_eq!(submarine.position, 0);
        assert_eq!(submarine.depth, 0);

        submarine.change_position(SubmarineMovement::Forward(4));
        assert_eq!(submarine.position, 4);
        assert_eq!(submarine.depth, 0);

        submarine.change_position(SubmarineMovement::Down(9));
        assert_eq!(submarine.position, 4);
        assert_eq!(submarine.depth, 9);

        submarine.change_position(SubmarineMovement::Up(3));
        assert_eq!(submarine.position, 4);
        assert_eq!(submarine.depth, 6);
    }
}
