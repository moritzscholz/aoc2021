use std::ops::RangeInclusive;
mod probe_launcher;
pub mod types;
use probe_launcher::ProbeLauncher;
use std::collections::HashMap;
use types::Velocity;

pub struct ProbeLauncherSimulation<T> {
    target_x: RangeInclusive<T>,
    target_y: RangeInclusive<T>,
}

impl ProbeLauncherSimulation<i32> {
    pub fn new(
        target_x: RangeInclusive<i32>,
        target_y: RangeInclusive<i32>,
    ) -> Self {
        Self { target_x, target_y }
    }

    // Returns initial velocity for highest shot and achieved height.
    pub fn initial_velocity_for_highest_shot(&self) -> (Velocity, i32) {
        let vx_range = 0..=*self.target_x.end();
        // Actually not sure if this is correct.
        let vy_range = 0..=*self.target_y.start() * (-1);

        let mut hitting_velocities: HashMap<i32, Velocity> = HashMap::new();

        for vx in vx_range.into_iter() {
            for vy in vy_range.clone().into_iter() {
                let initial_v = Velocity::from((vx, vy));
                let mut highest_y: i32 = 0;
                let mut launcher = ProbeLauncher::new(initial_v);

                loop {
                    let position = launcher.step();
                    highest_y = highest_y.max(position.y);

                    // Target hit
                    if self.target_x.contains(&position.x)
                        && self.target_y.contains(&position.y)
                    {
                        hitting_velocities.insert(highest_y, initial_v);
                        break;
                    }

                    // Overshoot -> Stop
                    if &position.x > self.target_x.end()
                        || &position.y < self.target_y.start()
                    {
                        break;
                    }

                    // Else: Continue with next step.
                }
            }
        }

        let highest_y = hitting_velocities
            .keys()
            .max()
            .expect("Did not find any initial velocity hitting the target!");
        (hitting_velocities[highest_y], *highest_y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_launcher_sim() {
        let sim = ProbeLauncherSimulation {
            target_x: 20..=30,
            target_y: -10..=-5,
        };
        let (_, y) = sim.initial_velocity_for_highest_shot();
        assert_eq!(y, 45);
    }
}
