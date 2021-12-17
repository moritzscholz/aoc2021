use super::types::{Position, Velocity};

pub struct ProbeLauncher {
    position: Position,
    velocity: Velocity,
}

impl ProbeLauncher {
    pub fn new(initial_v: Velocity) -> Self {
        Self {
            position: Position::from((0, 0)),
            velocity: initial_v,
        }
    }

    /*
    Performs a simulation step and returns the position after that step.

    Instructions:
    On each step, these changes occur in the following order:
        The probe's x position increases by its x velocity.
        The probe's y position increases by its y velocity.
        Due to drag, the probe's x velocity changes by 1 toward the value 0; that is, it decreases by 1 if it is greater than 0, increases by 1 if it is less than 0, or does not change if it is already 0.
        Due to gravity, the probe's y velocity decreases by 1.
     */
    pub fn step(&mut self) -> &Position {
        // The probe's x position increases by its x velocity.
        self.position.x += self.velocity.x;

        // The probe's y position increases by its y velocity.
        self.position.y += self.velocity.y;

        // Due to drag, the probe's x velocity changes by 1 toward the value 0.
        self.velocity.x -= self.velocity.x.signum();

        // Due to gravity, the probe's y velocity decreases by 1.
        self.velocity.y -= 1;

        &self.position
    }
}
