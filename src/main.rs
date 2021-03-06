mod file_handler;
mod sonar_depth;
mod submarine;
use crate::{
    crab_submarines::FuelBurnRate, submarine::Submarine,
    transparent_origami::TransparentPaper,
};
mod binary_diagnostic;
mod cave_floor_scanner;
mod cave_navigation;
mod crab_submarines;
mod lanternfish;
mod squid_bingo;
mod transparent_origami;
mod trench_exploration;

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
        "Solution for day 4: winning board's score * last number = {:?}. ",
        result.answer()
    );
    let mut game2 = squid_bingo::BingoGame::build_from("data/day4/input.txt");
    let result_end = game2
        .simulate_until_end()
        .unwrap_or_else(|| panic!("Game did not finish. {:#?}", game2));
    println!(
        "Last board to win, score * last number = {:?}!",
        result_end.answer()
    );

    // Day 5

    // Day 6
    let mut colony =
        lanternfish::LanternfishColony::from_file("data/day6/input.txt");
    colony.simulate_steps(80);
    print!(
        "Solution for day 6: There are {} lanternfish after 80 days. ",
        colony.size()
    );
    colony.simulate_steps(256 - 80);
    println!(
        "Part 2: There are {} lanternfish after 256 days.",
        colony.size()
    );

    // Day 7
    let fleet =
        crab_submarines::CrabSubmarineFleet::from_file("data/day7/input.txt");
    print!(
        "Solution for day 7: Position / fuel cost: {:?}. ",
        fleet.ideal_position_and_fuel(FuelBurnRate::Constant)
    );
    println!(
        "Part 2, with increasing fuel-rate: {:?}. ",
        fleet.ideal_position_and_fuel(FuelBurnRate::Increasing)
    );

    // Day 8

    // Day 9
    let scanner =
        cave_floor_scanner::CaveFloorScanner::from_file("data/day9/input.txt");
    print!("Solution for day 9: Cave risk is {}. ", scanner.cave_risk());
    println!();

    // Day 13
    let mut paper = TransparentPaper::from_file("data/day13/input.txt");
    paper.fold();
    print!(
        "Solution for day 13: Paper has {} markings after first fold.",
        paper.num_marked()
    );
    println!();
    println!("Final folded paper:");
    while paper.fold() {}
    paper.dump();

    // Day 15
    let cave = cave_navigation::Cave::from_grid_file("data/day15/input.txt");
    print!("Lowest total risk is {}. ", cave.lowest_total_risk());
    println!();

    // Day 17
    let sim = trench_exploration::ProbeLauncherSimulation::new(
        185..=221,
        -122..=-74,
    );
    let (highest_y, v_count) = sim.initial_velocity_for_highest_shot();
    print!("Highest shot reaches height {}. ", highest_y);
    println!(
        "There are {} initial velocity values hitting the target area.",
        v_count
    );
}
