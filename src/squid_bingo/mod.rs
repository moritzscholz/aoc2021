mod bingo_board;
use bingo_board::BingoBoard;
use std::path::Path;

use crate::file_handler::read_all;

pub struct BingoGame {
    // Boards of all players in the game
    boards: Vec<BingoBoard>,
    // Numbers which were drawn: 7, 4, 9, 5, ...
    drawn_numbers: Vec<u32>,
}

pub struct BingoGameResult {
    winning_board_score: u32,
    last_drawn_number: u32,
}

impl BingoGame {
    /// Parses the contents of the given `file` and creates a new bingo game.
    /// The file has to have a comma-separated list of numbers in the first
    /// line. These are the numbers that are drawn in the game.
    ///
    /// After that, there need to be bingo boards in the format described in
    /// `BingoBoard`. They have to be separated by empty lines.
    ///
    /// See data/day4/test.txt for an example.
    pub fn build_from<P>(file: P) -> BingoGame
    where
        P: AsRef<Path>,
    {
        let game_data = read_all(file).expect("Could not read game file.");

        let drawn_numbers: Vec<u32> = game_data
            .lines()
            .next()
            .expect("Could not read first line of game file!")
            .split(',')
            .map(|number_str| number_str.parse().unwrap())
            .collect();

        /*
            Boards are all separated by an empty line.
            The first line of the file is the drawn numbers, so we skip it.
        */
        let boards_data = game_data.split("\n\n").skip(1);

        let boards = boards_data
            .map(|board_data| {
                let lines = board_data
                    .lines()
                    .map(|line| line.to_string())
                    .collect::<Vec<_>>();
                BingoBoard::build_from(lines)
            })
            .collect();

        BingoGame {
            drawn_numbers,
            boards,
        }
    }

    pub fn simulate(&mut self) -> Option<BingoGameResult> {
        // Simulate all boards
        for number in &self.drawn_numbers {
            for board in self.boards.iter_mut() {
                board.mark_number(number);

                if board.has_won() {
                    return Some(BingoGameResult {
                        winning_board_score: board.score(),
                        last_drawn_number: *number,
                    });
                }
            }
        }

        None
    }
}

impl BingoGameResult {
    /// Provides answer to the challenge of day 4.
    pub fn answer(&self) -> u32 {
        self.winning_board_score * self.last_drawn_number
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_bingo_game() {
        let game = BingoGame::build_from("data/day4/test.txt");
        assert_eq!(game.boards.len(), 3);
        assert_eq!(
            game.drawn_numbers,
            vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15,
                25, 12, 22, 18, 20, 8, 19, 3, 26, 1
            ]
        );
    }

    #[test]
    fn simulate_game() {
        let mut game = BingoGame::build_from("data/day4/test.txt");
        let result = game.simulate().unwrap();
        assert_eq!(result.winning_board_score, 188);
        assert_eq!(result.last_drawn_number, 24);
        assert_eq!(result.answer(), 4512);
    }
}
