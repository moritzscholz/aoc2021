mod sonar_depth;
use crate::sonar_depth::{count_increases, count_increases_sliding};

fn main() {
    println!("Increases: {:?}", count_increases("input.txt"));
    println!(
        "Increases of sliding sums: {:?}",
        count_increases_sliding("input.txt")
    );
}
