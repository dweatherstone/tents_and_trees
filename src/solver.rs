use std::{
    cmp::{max, min},
    fmt,
};

use crate::{
    backtracker::Config,
    board::{Board, CellType},
};

/// This struct holds the configuration of a step in solving a Tents and Trees Puzzle
pub struct TentsAndTreesConfig {
    pub board: Board,
    row: usize,
    col: usize,
}

impl TentsAndTreesConfig {
    pub fn new(board: &Board) -> Self {
        TentsAndTreesConfig {
            board: board.clone(),
            row: 0,
            col: 0,
        }
    }
    fn from(old_config: &TentsAndTreesConfig, row: usize, col: usize) -> TentsAndTreesConfig {
        let mut new_board = old_config.board.clone();
        new_board.board[row][col] = CellType::Tent;
        TentsAndTreesConfig {
            board: new_board,
            row,
            col,
        }
    }
}

impl Config for TentsAndTreesConfig {
    fn successors(&self) -> Vec<Self> {
        let mut successors = Vec::new();
        // Loop through all the trees on board, and position a tent for each.
        // Step 1, get a vec of all the tent positions
        let tree_positions = self.board.get_tree_positions();
        for (tree_row_idx, tree_col_idx, possible_directions) in tree_positions {
            if !self
                .board
                .does_surrounding_have_tent(tree_row_idx, tree_col_idx)
            {
                for direction in possible_directions.iter() {
                    match self.board.get_possible_tent_position(
                        tree_row_idx,
                        tree_col_idx,
                        direction,
                    ) {
                        Ok((tent_row_idx, tent_col_idx)) => {
                            if self.board.is_valid_peek(tent_row_idx, tent_col_idx) {
                                successors.push(TentsAndTreesConfig::from(
                                    self,
                                    tent_row_idx,
                                    tent_col_idx,
                                ));
                            }
                        }
                        Err(e) => {
                            eprintln!("{}", e);
                            continue;
                        }
                    }
                }
            }
        }

        successors
    }

    /// Check if the config is valid. This assumes that other tent locations are valid, and only
    /// checks the most recently added tent to see if it conflicts with other tents.
    fn is_valid(&self) -> bool {
        // Find the minimum and maximum row and column values based on the tent position to check
        // and the size bounds of the board.
        // We will check the maximum of 8 surrounding cells to see if the contain a tent. If so, return false
        let row_min = max(0i32, self.row as i32 - 1) as usize;
        let row_max = min(self.row + 1, self.board.row_count - 1);
        let col_min = max(0i32, self.col as i32 - 1) as usize;
        let col_max = min(self.col + 1, self.board.col_count - 1);
        for r in row_min..=row_max {
            for c in col_min..=col_max {
                // If we are in the position of the last added tent, then don't check
                if r == self.row && c == self.col {
                    continue;
                }
                if self.board.get_celltype(r, c) == CellType::Tent {
                    return false;
                }
            }
        }
        // Check the row and column clues to make sure that we haven't added too many tents
        if let Some(row_clue) = self.board.row_clues[self.row].get_clue() {
            let row_count = self.board.board[self.row]
                .iter()
                .filter(|x| *x == &CellType::Tent)
                .count();
            if row_count > row_clue {
                return false;
            }
        }
        if let Some(col_clue) = self.board.col_clues[self.col].get_clue() {
            let col_count = self
                .board
                .get_column(self.col)
                .iter()
                .filter(|x| *x == &CellType::Tent)
                .count();
            if col_count > col_clue {
                return false;
            }
        }
        // No surrounding tents, so this config is valid
        true
    }

    fn is_goal(&self) -> bool {
        if !self.is_valid() {
            return false;
        }
        self.board.is_complete()
    }
}

impl fmt::Display for TentsAndTreesConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.board)
    }
}
