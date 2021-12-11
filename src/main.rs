mod file_handler;
mod sonar_depth;
mod submarine;
use crate::submarine::Submarine;

fn main() {
    // Day 1
    println!(
        "Solution for day 1: {:?} increases, {:?} sliding window sum increases.",
        sonar_depth::count_increases("data/day1/input.txt"),
        sonar_depth::count_increases_sliding("data/day1/input.txt")
    );

    // Day 2
    let mut submarine = submarine::SimpleSubmarine::new();
    submarine.change_position_from("data/day2/input.txt");
    print!(
        "Solution for day 2: final horizontal position * final depth = {:?}",
        submarine.position_hash()
    );
}
