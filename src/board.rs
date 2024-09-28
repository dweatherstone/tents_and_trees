use std::{
    cmp::{max, min},
    fmt::{Debug, Display, Write},
};

use crate::AppError;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CellType {
    Unknown,
    Empty,
    Tent,
    Tree,
}

#[derive(PartialEq, Clone)]
pub struct Clue {
    clue: Option<usize>,
    //is_complete: bool,
}

impl Clue {
    pub fn get_clue(&self) -> Option<usize> {
        self.clue
    }
    /*
    pub fn is_complete(&self) -> bool {
        self.is_complete
    }

    pub fn set_complete(&mut self) {
        self.is_complete = true;
    }
     */
}

pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
pub enum BoardErrorType {
    NoTreeFound(usize, usize),
    ImpossibleTentPosition(usize, usize),
}

impl Display for BoardErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoardErrorType::NoTreeFound(r, c) => {
                write!(
                    f,
                    "{}",
                    format_args!("No tree found at position ({}, {})", r, c)
                )
            }
            BoardErrorType::ImpossibleTentPosition(r, c) => write!(
                f,
                "{}",
                format_args!("Impossible for tent to be at position ({}, {})", r, c)
            ),
        }
    }
}

#[derive(Default, Clone)]
pub struct Board {
    pub board: Vec<Vec<CellType>>,
    pub col_clues: Vec<Clue>,
    pub row_clues: Vec<Clue>,
    pub col_count: usize,
    pub row_count: usize,
}

impl Board {
    pub fn new(
        board: Vec<Vec<CellType>>,
        col_clues: Vec<Option<usize>>,
        row_clues: Vec<Option<usize>>,
    ) -> Self {
        let row_count = col_clues.len();
        let col_count = row_clues.len();

        Board {
            board,
            col_clues: col_clues
                .iter()
                .map(|clue| Clue {
                    clue: *clue,
                    //is_complete: clue.is_none(),
                })
                .collect(),
            row_clues: row_clues
                .iter()
                .map(|clue| Clue {
                    clue: *clue,
                    //is_complete: clue.is_none(),
                })
                .collect(),
            col_count,
            row_count,
        }
    }

    pub fn get_celltype(&self, row: usize, col: usize) -> CellType {
        self.board[row][col]
    }

    pub fn verify_setup(&self) -> bool {
        let row_counts = self.board.iter().all(|row| row.len() == self.row_count);
        let col_counts = self.board.len() == self.col_count;
        row_counts & col_counts
    }

    pub fn is_complete(&self) -> bool {
        // self.col_clues.iter().all(|c| c.is_complete())
        //     & self.row_clues.iter().all(|r| r.is_complete())
        self.col_clues.iter().enumerate().all(|(col_idx, clue)| {
            if let Some(clue_val) = clue.clue {
                clue_val
                    == self
                        .get_column(col_idx)
                        .iter()
                        .filter(|val| *val == &CellType::Tent)
                        .count()
            } else {
                true
            }
        }) & self.row_clues.iter().enumerate().all(|(row_idx, clue)| {
            if let Some(clue_val) = clue.clue {
                clue_val
                    == self.board[row_idx]
                        .iter()
                        .filter(|val| *val == &CellType::Tent)
                        .count()
            } else {
                true
            }
        })
    }

    pub fn get_column(&self, col: usize) -> Vec<CellType> {
        self.board.iter().map(|row| row[col]).collect()
    }

    pub fn does_surrounding_have_tent(&self, row: usize, col: usize) -> bool {
        // Check West
        if col > 0 && self.board[row][col - 1] == CellType::Tent {
            return true;
        }
        // Check North
        if row > 0 && self.board[row - 1][col] == CellType::Tent {
            return true;
        }
        // Check East
        if col < self.col_count - 1 && self.board[row][col + 1] == CellType::Tent {
            return true;
        }
        // Check South
        if row < self.row_count - 1 && self.board[row + 1][col] == CellType::Tent {
            return true;
        }
        false
    }

    pub fn is_valid_peek(&self, row: usize, col: usize) -> bool {
        let mut board_copy = self.board.clone();
        board_copy[row][col] = CellType::Tent;
        let row_min = max(0i32, row as i32 - 1) as usize;
        let row_max = min(row + 1, self.row_count - 1);
        let col_min = max(0i32, col as i32 - 1) as usize;
        let col_max = min(col + 1, self.col_count - 1);
        for r in row_min..=row_max {
            for c in col_min..=col_max {
                // If we are in the position of the last added tent, then don't check
                if r == row && c == col {
                    continue;
                }
                if board_copy[r][c] == CellType::Tent {
                    return false;
                }
            }
        }

        let row_counts = board_copy.iter().all(|row| row.len() == self.row_count);
        let col_counts = board_copy.len() == self.col_count;
        row_counts & col_counts
    }

    fn does_surrounding_have_tree(&self, row: usize, col: usize) -> bool {
        // Check West
        if col > 0 && self.board[row][col - 1] == CellType::Tree {
            return true;
        }
        // Check North
        if row > 0 && self.board[row - 1][col] == CellType::Tree {
            return true;
        }
        // Check East
        if col < self.col_count - 1 && self.board[row][col + 1] == CellType::Tree {
            return true;
        }
        // Check South
        if row < self.row_count - 1 && self.board[row + 1][col] == CellType::Tree {
            return true;
        }
        false
    }

    pub fn set_mandatory_empty(&mut self) {
        // Set all board values to CellType::Empty if the row_clue = 0
        for (row_num, row_clue) in self.row_clues.iter().enumerate() {
            if row_clue.clue.unwrap_or(1) == 0 {
                self.board[row_num]
                    .iter_mut()
                    .filter(|x| x == &&CellType::Unknown)
                    .for_each(|x| *x = CellType::Empty);
            }
        }

        // Set all board values to CellType::Empty if the col_clue = 0
        for (col_num, col_clue) in self.col_clues.iter().enumerate() {
            if col_clue.clue.unwrap_or(1) == 0 {
                //self.update_column(col_num, CellType::Empty);
                for row in self.board.iter_mut() {
                    if col_num < row.len() && row[col_num] == CellType::Unknown {
                        row[col_num] = CellType::Empty;
                    }
                }
            }
        }

        // Set all board values to CellType::Empty if there is no CellType::Tree to the North, South, East or West.
        for row in 0..self.row_count {
            for col in 0..self.col_count {
                if self.board[row][col] == CellType::Unknown
                    && !self.does_surrounding_have_tree(row, col)
                {
                    self.board[row][col] = CellType::Empty;
                }
            }
        }
    }

    pub fn get_tree_positions(&self) -> Vec<(usize, usize, Vec<Direction>)> {
        let mut tree_positions = Vec::new();
        for (row_idx, row) in self.board.iter().enumerate() {
            for (col_idx, value) in row.iter().enumerate() {
                if value == &CellType::Tree {
                    tree_positions.push((
                        row_idx,
                        col_idx,
                        self.get_possible_tent_directions(row_idx, col_idx),
                    ));
                }
            }
        }

        tree_positions
    }

    fn get_possible_tent_directions(&self, tree_row: usize, tree_col: usize) -> Vec<Direction> {
        let mut possible_directions = Vec::new();
        // Check West
        if tree_col > 0 && self.board[tree_row][tree_col - 1] == CellType::Unknown {
            possible_directions.push(Direction::West);
        }
        // Check North
        if tree_row > 0 && self.board[tree_row - 1][tree_col] == CellType::Unknown {
            possible_directions.push(Direction::North);
        }
        // Check East
        if tree_col < self.col_count - 1 && self.board[tree_row][tree_col + 1] == CellType::Unknown
        {
            possible_directions.push(Direction::East);
        }
        // Check South
        if tree_row < self.row_count - 1 && self.board[tree_row + 1][tree_col] == CellType::Unknown
        {
            possible_directions.push(Direction::South);
        }

        possible_directions
    }

    pub fn get_possible_tent_position(
        &self,
        tree_row: usize,
        tree_col: usize,
        direction: &Direction,
    ) -> Result<(usize, usize), AppError> {
        // Confirm that there is a tree in the specified position
        if self.board[tree_row][tree_col] != CellType::Tree {
            return Err(AppError::BoardError(BoardErrorType::NoTreeFound(
                tree_row, tree_col,
            )));
        }
        // Get the position of the possible tent, as long as the position is not out of bounds
        let (tent_row, tent_col) = match direction {
            Direction::North => {
                if tree_row == 0 {
                    return Err(AppError::BoardError(
                        BoardErrorType::ImpossibleTentPosition(tree_row, tree_col),
                    ));
                }
                (tree_row - 1, tree_col)
            }
            Direction::West => {
                if tree_col == 0 {
                    return Err(AppError::BoardError(
                        BoardErrorType::ImpossibleTentPosition(tree_row, tree_col),
                    ));
                }
                (tree_row, tree_col - 1)
            }
            Direction::South => {
                if tree_row >= self.row_count - 1 {
                    return Err(AppError::BoardError(
                        BoardErrorType::ImpossibleTentPosition(tree_row, tree_col),
                    ));
                }
                (tree_row + 1, tree_col)
            }
            Direction::East => {
                if tree_col >= self.col_count - 1 {
                    return Err(AppError::BoardError(
                        BoardErrorType::ImpossibleTentPosition(tree_row, tree_col),
                    ));
                }
                (tree_row, tree_col + 1)
            }
        };
        Ok((tent_row, tent_col))
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("   |")?;
        for clue in &self.col_clues {
            // let is_complete_marker = {
            //     if clue.is_complete() {
            //         "X"
            //     } else {
            //         " "
            //     }
            // };
            let is_complete_marker = " ";
            match clue.get_clue() {
                Some(val) => f.write_fmt(format_args!("{:>2}{}", val, is_complete_marker))?,
                None => f.write_str(" _ ")?,
            }
        }
        f.write_char('\n')?;
        f.write_str("---|")?;
        for _ in 0..self.col_clues.len() {
            f.write_str("---")?;
        }
        f.write_char('\n')?;
        for (row_num, row) in self.board.iter().enumerate() {
            // let is_complete_marker = if self.row_clues[row_num].is_complete() {
            //     "X"
            // } else {
            //     " "
            // };
            let is_complete_marker = " ";
            if let Some(val) = self.row_clues[row_num].get_clue() {
                f.write_fmt(format_args!("{:>2}{}|", val, is_complete_marker))?;
            } else {
                f.write_str(" _ |")?;
            }
            for val in row.iter() {
                let ct_repr = match val {
                    CellType::Empty => " E ",
                    CellType::Unknown => " - ",
                    CellType::Tent => " X ",
                    CellType::Tree => " T ",
                };
                f.write_str(ct_repr)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        if self.row_count != other.row_count
            || self.col_count != other.col_count
            || self.board.len() != other.board.len()
        {
            return false;
        }
        for row in 0..self.col_count {
            for col in 0..self.row_count {
                if self.board[row][col] != other.board[row][col] {
                    return false;
                }
            }
        }
        true
    }
}

impl Eq for Board {}
