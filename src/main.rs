mod sonar_depth;
use crate::sonar_depth::count_increases;

fn main() {
    println!("{:?}", count_increases("input.txt"));
}
