mod file_handler;
mod sonar_depth;
mod submarine;
use crate::submarine::Submarine;
mod binary_diagnostic;
mod squid_bingo;

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
        "Solution for day 2: final horizontal position * final depth = {:?}. ",
        submarine.position_hash()
    );
    let mut aimed_submarine = submarine::AimedSubmarine::new();
    aimed_submarine.change_position_from("data/day2/input.txt");
    println!("Aimed submarine: {:?}.", aimed_submarine.position_hash());

    // Day 3
    let diagnostics_result =
        binary_diagnostic::DiagnosticsResult::diagnose("data/day3/input.txt");
    print!(
        "Solution for day 3: Power consumption = {:?}",
        diagnostics_result.power_consumption()
    );
    println!();

    // Day 4
    let mut game = squid_bingo::BingoGame::build_from("data/day4/input.txt");
    let result = game.simulate().unwrap();
    print!(
        "Solution for day 4: winning board's score * last number = {:?}",
        result.answer()
    );
    println!();
}
