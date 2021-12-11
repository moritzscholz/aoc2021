mod sonar_depth;

fn main() {
    println!(
        "Solution day 1: {:?} increases, {:?} sliding window sum increases.",
        sonar_depth::count_increases("data/day1/input.txt"),
        sonar_depth::count_increases_sliding("data/day1/input.txt")
    );
}
