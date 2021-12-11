#[derive(Clone, Copy, Debug)]
struct BingoNumber {
    number: u32,
    marked: bool,
}

#[derive(Debug)]
pub struct BingoBoard {
    board: Vec<Vec<BingoNumber>>,
}

impl BingoNumber {
    fn new(number: u32) -> BingoNumber {
        BingoNumber {
            number,
            marked: false,
        }
    }

    fn mark(&mut self) {
        self.marked = true
    }
}

impl BingoBoard {
    /// Creates a new bingo board from a block
    /// of text `lines` in the following format:
    /// ```
    /// 22 13 17 11  0
    ///  8  2 23  4 24
    /// 21  9 14 16  7
    ///  6 10  3 18  5
    ///  1 12 20 15 19
    /// ```
    pub fn build_from(lines: Vec<String>) -> BingoBoard {
        BingoBoard {
            board: {
                lines
                    .into_iter()
                    .map(|line| {
                        line.split_whitespace()
                            .map(|number_str| {
                                let number =
                                    number_str.parse().unwrap_or_default();
                                BingoNumber::new(number)
                            })
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>()
            },
        }
    }

    #[allow(dead_code)]
    fn row(&self, index: usize) -> &Vec<BingoNumber> {
        &self.board[index]
    }

    pub fn mark_number(&mut self, drawn_number: &u32) {
        for number in self.board.iter_mut().flatten() {
            if number.number == *drawn_number {
                number.mark()
            }
        }
    }

    pub fn has_won(&self) -> bool {
        let has_full_row = self
            .board
            .iter()
            .any(|row| row.iter().all(|number| number.marked));
        if has_full_row {
            return true;
        }

        let cols = self.board[0].len();
        for col_idx in 0..cols {
            let full_col = self.board.iter().all(|row| row[col_idx].marked);
            if full_col {
                return true;
            }
        }

        false
    }

    pub fn score(&self) -> u32 {
        self.board
            .iter()
            .flatten()
            .filter(|number| !number.marked)
            .map(|number| number.number)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::file_handler::read_lines;

    use super::*;

    #[test]
    fn parse_bingo_board() {
        let lines = read_lines("data/day4/test_board.txt")
            .unwrap()
            .map(|line| line.unwrap())
            .collect::<Vec<_>>();

        let board = BingoBoard::build_from(lines);
        assert_eq!(
            board.row(0).iter().map(|bn| bn.number).collect::<Vec<_>>(),
            vec![22, 13, 17, 11, 0]
        );

        assert_eq!(
            board.row(1).iter().map(|bn| bn.number).collect::<Vec<_>>(),
            vec![8, 2, 23, 4, 24]
        );

        assert_eq!(
            board.row(2).iter().map(|bn| bn.number).collect::<Vec<_>>(),
            vec![21, 9, 14, 16, 7]
        );

        assert_eq!(
            board.row(3).iter().map(|bn| bn.number).collect::<Vec<_>>(),
            vec![6, 10, 3, 18, 5]
        );

        assert_eq!(
            board.row(4).iter().map(|bn| bn.number).collect::<Vec<_>>(),
            vec![1, 12, 20, 15, 19]
        );
    }
}
