use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp::{max, min};

#[derive(Clone, Copy, Debug, PartialEq)]
enum CellStatus {
    Empty,
    Unknown,
    Tree,
    Tent,
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(PartialEq)]
pub struct Board{
    column_counts: Vec<Option<i8>>,
    row_counts: Vec<Option<i8>>,
    board: Vec<Vec<CellStatus>>,
    size: usize,
}

impl Board {
    pub fn check_and_complete_all(&mut self) -> bool {
        let mut rows_changed = false;
        let mut cols_changed = false;
        for row_num in 0..self.size {
            let this_row_changed = self.check_and_complete_row(row_num);
            rows_changed = rows_changed || this_row_changed;
        }
        for col_num in 0..self.size {
            let this_col_changed = self.check_and_complete_col(col_num);
            cols_changed = cols_changed || this_col_changed;
        }
        return rows_changed || cols_changed;
    }

    pub fn empty_no_surrounding_trees(&mut self) -> bool {
        let mut has_changed = false;
        for row_num in 0..self.size {
            for col_num in 0..self.size {
                if self.board[row_num][col_num] != CellStatus::Unknown {
                    continue;
                }
                if !self.surrounding_cells_has_tree(row_num, col_num) {
                    self.board[row_num][col_num] = CellStatus::Empty;
                    has_changed = true;
                }
            }
        }

        has_changed
    }

    pub fn only_one_place_for_tent(&mut self) -> bool {
        let mut has_changed = false;
        for row_num in 0..self.size {
            for col_num in 0..self.size {
                if self.board[row_num][col_num] == CellStatus::Tree {
                    let this_changed = self.has_placed_tent(row_num, col_num);
                    if this_changed && !has_changed {
                        has_changed = true;
                    }
                }
            }
        }
        has_changed
    }

    pub fn only_space_for_tents(&mut self) -> bool {
        let mut rows_changed = false;
        let mut cols_changed = false;
        for row_num in 0..self.size {
            let this_row_changed = self.only_space_for_tents_row(row_num);
            rows_changed = rows_changed || this_row_changed;
        }
        for col_num in 0..self.size {
            let this_col_changed = self.only_space_for_tents_col(col_num);
            cols_changed = cols_changed || this_col_changed;
        }
        return rows_changed || cols_changed;
    }

    pub fn is_complete(&self) -> bool {
        for row_num in 0..self.size {
            if !self.is_row_complete(row_num) {
                return false;
            }
        }
        return true;
    }

    pub fn get_board_state(&self) -> String {
        let mut print_string = String::from(" | ");
        for col_count in self.column_counts.iter() {
            let col_val = match col_count {
                Some(val) => val.to_string(),
                None => String::from("-"),
            };
            print_string.push_str(&col_val);
            print_string.push_str(" ");
        }
        print_string.push_str("\n");
        print_string.push_str(&"--".repeat(self.size+1));
        print_string.push_str("\n");
        for row_num in 0..self.size {
            let row_val = match self.row_counts[row_num] {
                Some(val) => val.to_string(),
                None => String::from("-"),
            };
            print_string.push_str(&row_val);
            print_string.push_str("| ");
            for status in self.board[row_num].iter() {
                let cell_string = match status {
                    CellStatus::Empty => "E",
                    CellStatus::Tent => "X",
                    CellStatus::Tree => "T",
                    CellStatus::Unknown => " ",
                };
                print_string.push_str(&cell_string);
                print_string.push_str(" ");
            }
            print_string.push_str("\n");
        }
        print_string
    }

    pub fn print_board(&self) {
        println!();
        print!("{}", self.get_board_state());
    }

    fn has_placed_tent(&mut self, row_num: usize, col_num: usize) -> bool {
        let mut empty_count = 0;
        let mut change_direction = Direction::Left;
        if row_num >= self.size || col_num >= self.size {
            panic!("Incorrect row or col number in parameter");
        }
        // Check left
        if col_num != 0 && self.board[row_num][col_num-1] == CellStatus::Unknown {
            empty_count += 1;
            change_direction = Direction::Left;
        }
        // Check right
        if col_num < self.size-1 && self.board[row_num][col_num+1] == CellStatus::Unknown {
            empty_count += 1;
            change_direction = Direction::Right;
        }
        // Check up
        if row_num != 0 && self.board[row_num-1][col_num] == CellStatus::Unknown {
            empty_count += 1;
            change_direction = Direction::Up;
        }
        // Check down
        if row_num < self.size-1 && self.board[row_num+1][col_num] == CellStatus::Unknown {
            empty_count += 1;
            change_direction = Direction::Down;
        }

        if empty_count == 1 {
            match change_direction {
                Direction::Left => {
                    self.board[row_num][col_num-1] = CellStatus::Tent;
                    self.surround_tent_with_empty(row_num, col_num-1);
                    return true;
                },
                Direction::Right => {
                    self.board[row_num][col_num+1] = CellStatus::Tent;
                    self.surround_tent_with_empty(row_num, col_num+1);
                    return true;
                },
                Direction::Up => {
                    self.board[row_num-1][col_num] = CellStatus::Tent;
                    self.surround_tent_with_empty(row_num-1, col_num);
                    return true;
                },
                Direction::Down => {
                    self.board[row_num+1][col_num] = CellStatus::Tent;
                    self.surround_tent_with_empty(row_num+1, col_num);
                    return true;
                }
            }
        } else {
            return false;
        }
    }

    fn surround_tent_with_empty(&mut self, row_num: usize, col_num: usize) {
        let start_row: usize = max(0, row_num as i8 - 1) as usize;
        let end_row: usize = min(self.size-1, row_num + 1);
        let start_col: usize = max(0, col_num as i8 - 1) as usize;
        let end_col: usize = min(self.size-1, col_num + 1);
        for row in start_row..end_row+1 {
            for col in start_col..end_col+1 {
                if self.board[row][col] == CellStatus::Unknown {
                    self.board[row][col] = CellStatus::Empty;
                }
            }
        }
    }

    fn surrounding_cells_has_tree(&self, row_num: usize, col_num: usize) -> bool {
        if row_num >= self.size || col_num >= self.size {
            panic!("Incorrect row or col number in parameter");
        }
        
        if self.board[row_num][col_num] != CellStatus::Unknown {
            return true;
        }
        // Check left
        if col_num != 0 && self.board[row_num][col_num-1] == CellStatus::Tree {
            return true;
        }
        // Check right
        if col_num < self.size-1 && self.board[row_num][col_num+1] == CellStatus::Tree {
            return true;
        }
        // Check up
        if row_num != 0 && self.board[row_num-1][col_num] == CellStatus::Tree {
            return true;
        }
        // Check down
        if row_num < self.size-1 && self.board[row_num+1][col_num] == CellStatus::Tree {
            return true;
        }

        return false;
    }

    fn is_row_complete(&self, row_num: usize) -> bool {
        let row = self.board.get(row_num).expect("The row does not exist!");
        for cell in row.iter() {
            if let CellStatus::Unknown = cell {
                return false;
            }
        }
        return true;
    }

    fn get_row_count(&self, row_num: usize) -> i8 {
        let mut count: i8 = 0;
        let row = self.board.get(row_num).expect("Row num not fount");
        for cell in row.iter() {
            if let CellStatus::Tent = cell {
                count += 1;
            }
        }
        count
    }

    fn check_and_complete_row(&mut self, row_num: usize) -> bool {
        let count = self.get_row_count(row_num);
        
        let row_count:i8 = match self.row_counts.get(row_num).expect("Cannot find row val") {
            Some(val) => *val,
            None => -1,
        };
        if row_count == -1 {
            return false;
        }
        let mut has_updated = false;
        if count == row_count {
            let row = self.board.get_mut(row_num).expect("Row num not fount");
            for cell in row.iter_mut() {
                if let CellStatus::Unknown = cell {
                    *cell = CellStatus::Empty;
                    has_updated = true;
                }
            }
        } 
        has_updated
    }

    fn only_space_for_tents_row(&mut self, row_num: usize) -> bool {
        let count = self.get_unknown_or_tent_row_count(row_num);
        let row_count = match self.row_counts.get(row_num).expect("Cannot find row val") {
            Some(val) => *val,
            None => -1,
        };
        if row_count == -1 {
            return false;
        }
        let mut has_updated = false;
        if count == row_count {
            for col_num in 0..self.size {
                if let CellStatus::Unknown = self.board[row_num][col_num] {
                    self.board[row_num][col_num] = CellStatus::Tent;
                    self.surround_tent_with_empty(row_num, col_num);
                    has_updated = true;
                }
            }
        }
        has_updated
    }

    fn get_unknown_or_tent_row_count(&self, row_num: usize) -> i8 {
        let mut count: i8 = 0;
        let row = self.board.get(row_num).expect("Row num not fount");
        for cell in row.iter() {
            if cell == &CellStatus::Unknown || cell == &CellStatus::Tent {
                count += 1;
            }
        }
        count
    }

    fn get_unknown_or_tent_col_count(&self, col_num: usize) -> i8 {
        let mut count: i8 = 0;
        let mut col: Vec<CellStatus> = Vec::new();
        for row in self.board.iter() {
            let cell = row.get(col_num).expect("Cannot find value!");
            col.push(*cell);
        }
        for cell in col.iter() {
            if cell == &CellStatus::Unknown || cell == &CellStatus::Tent {
                count += 1;
            }
        }
        count
    }

    // fn is_col_complete(&self, col_num: usize) -> bool {
    //     let mut col: Vec<CellStatus> = Vec::new();
    //     for row in self.board.iter() {
    //         let cell = row.get(col_num).expect("Cannot find value!");
    //         col.push(*cell);
    //     }
    //     for cell in col.iter() {
    //         if let CellStatus::Unknown = cell {
    //             return false;
    //         }
    //     }
    //     return true;
    // }

    fn get_col_count(&self, col_num: usize) -> i8 {
        let mut count: i8 = 0;
        let mut col: Vec<CellStatus> = Vec::new();
        for row in self.board.iter() {
            let cell = row.get(col_num).expect("Cannot find value!");
            col.push(*cell);
        }
        for cell in col.iter() {
            if let CellStatus::Tent = cell {
                count += 1;
            }
        }
        count
    }

    fn check_and_complete_col(&mut self, col_num: usize) -> bool {
        let count = self.get_col_count(col_num);
        let col_count:i8;
        match self.column_counts.get(col_num).expect("Cannot find col val") {
            Some(val) => col_count = *val,
            None => return false
        }
        let mut has_updated = false;
        if count == col_count {
            for row_num in 0..self.size {
                if let CellStatus::Unknown = self.board[row_num][col_num] {
                    self.board[row_num][col_num] = CellStatus::Empty;
                    has_updated = true;
                }
            }
        } 
        has_updated
    }

    fn only_space_for_tents_col(&mut self, col_num: usize) -> bool {
        let count = self.get_unknown_or_tent_col_count(col_num);
        let col_count = match self.column_counts.get(col_num).expect("Cannot find col val") {
            Some(val) => *val,
            None => -1,
        };
        if col_count == -1 {
            return false;
        }
        let mut has_updated = false;
        if count == col_count {
            for row_num in 0..self.size {
                if let CellStatus::Unknown = self.board[row_num][col_num] {
                    self.board[row_num][col_num] = CellStatus::Tent;
                    self.surround_tent_with_empty(row_num, col_num);
                    has_updated = true;
                }
            }
        }
        has_updated
    }

}

pub fn read_board(filename: &String) -> Board {
    // open the file in read-only mode
    let file = File::open(filename).unwrap();
    // Read the file line by line
    let mut col_counts: Vec<Option<i8>> = Vec::new();
    let mut row_counts: Vec<Option<i8>> = Vec::new();
    let mut size: usize = 0;
    let mut board: Vec<Vec<CellStatus>> = Vec::new();
    for (line_num, line) in BufReader::new(file).lines().enumerate() {
        if line.is_err() {
            panic!("Cannot read line of file {}", filename);
        }
        let line = line.unwrap();
        let mut items: Vec<&str> = line.split(',').collect();
        if line_num == 0 {
            _ = items.remove(0);
            size = items.len();
            for val in items {
                match val {
                    "0"|"1"|"2"|"3"|"4"|"5"|"6"|"7"|"8"|"9" => {
                        let val_num: i8 = val.parse::<i8>().expect("Value is expected to be an int.");
                        col_counts.push(Some(val_num));
                    },
                    "-" => col_counts.push(None),
                    _ => panic!("Unexpected value in column counts")
                }
            }
        } else {
            let mut row: Vec<CellStatus> = Vec::new();
            let row_count = items[0];
            match row_count {
                "0"|"1"|"2"|"3"|"4"|"5"|"6"|"7"|"8"|"9" => {
                    let val_num: i8 = row_count.parse::<i8>().expect("Value is expected to be an int.");
                    row_counts.push(Some(val_num));
                },
                "-" => row_counts.push(None),
                _ => panic!("Unexpected value in column counts")
            }
            _ = items.remove(0);
            for val in items {
                match val {
                    "t" => {
                        row.push(CellStatus::Tree);
                    },
                    "-" => {
                        row.push(CellStatus::Unknown);
                    },
                    _ => panic!("Unknown cell value when reading file"),
                }
            }
            board.push(row);
        }
    }
    Board {
        size,
        column_counts: col_counts,
        row_counts,
        board,
    }

} 


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn row_is_complete() {
        let test_row: usize = 0;
        let mut test_board = create_test_board();
        test_board.board[test_row][1] = CellStatus::Empty;
        test_board.board[test_row][2] = CellStatus::Empty;
        assert!(test_board.is_row_complete(test_row));
    }

    #[test]
    fn row_is_not_complete() {
        let test_row: usize = 2;
        let test_board = create_test_board();
        assert!(!test_board.is_row_complete(test_row));
    }

    // #[test]
    // fn col_is_complete() {
    //     let test_col: usize = 0;
    //     let mut test_board = create_test_board();
    //     test_board.board[1][test_col] = CellStatus::Empty;
    //     test_board.board[2][test_col] = CellStatus::Empty;
    //     assert!(test_board.is_col_complete(test_col));
    // }

    // #[test]
    // fn col_is_not_complete() {
    //     let test_col: usize = 2;
    //     let test_board = create_test_board();
    //     assert!(!test_board.is_col_complete(test_col));
    // }

    #[test]
    fn row_check_and_complete_true() {
        let test_row: usize = 0;
        let mut test_board = create_test_board();
        let result = test_board.check_and_complete_row(test_row);
        let expected_row = vec![CellStatus::Tent, CellStatus::Empty, CellStatus::Empty];
        assert_eq!(test_board.board[test_row], expected_row);
        assert!(result);
    }

    #[test]
    fn row_check_and_complete_false() {
        let test_row: usize = 2;
        let mut test_board = create_test_board();
        let result = test_board.check_and_complete_row(test_row);
        let expected_row = vec![CellStatus::Unknown, CellStatus::Unknown, CellStatus::Tent];
        assert_eq!(test_board.board[test_row], expected_row);
        assert!(!result);
    }

    #[test]
    fn col_check_and_complete_true() {
        let test_col: usize = 0;
        let mut test_board = create_test_board();
        let result = test_board.check_and_complete_col(test_col);
        let expected_col = vec![CellStatus::Tent, CellStatus::Empty, CellStatus::Empty];
        let result_col = vec![
            test_board.board[0][test_col],
            test_board.board[1][test_col],
            test_board.board[2][test_col],
        ];
        assert_eq!(result_col, expected_col);
        assert!(result);
    }

    #[test]
    fn col_check_and_complete_false() {
        let test_col: usize = 2;
        let mut test_board = create_test_board();
        let result = test_board.check_and_complete_col(test_col);
        let expected_col = vec![CellStatus::Unknown, CellStatus::Unknown, CellStatus::Tent];
        let result_col = vec![
            test_board.board[0][test_col],
            test_board.board[1][test_col],
            test_board.board[2][test_col],
        ];
        assert_eq!(result_col, expected_col);
        assert!(!result);
    }

    #[test]
    fn board_test1() {
        let test_board = read_board(&String::from("./test1.txt"));
        let expected_col_counts: Vec<Option<i8>> = vec![Some(3),Some(0),Some(1),Some(2),Some(0),Some(1)];
        let expected_row_counts: Vec<Option<i8>> = vec![Some(1),Some(2),Some(0),Some(2),Some(0),Some(2)];
        let expected_size: usize = 6;
        let expected_board: Vec<Vec<CellStatus>> = vec![
            vec![CellStatus::Tree, CellStatus::Unknown, CellStatus::Unknown, CellStatus::Unknown, CellStatus::Unknown, CellStatus::Tree],
            vec![CellStatus::Unknown, CellStatus::Unknown, CellStatus::Unknown, CellStatus::Tree, CellStatus::Unknown, CellStatus::Unknown],
            vec![CellStatus::Tree, CellStatus::Unknown, CellStatus::Unknown, CellStatus::Unknown, CellStatus::Unknown, CellStatus::Unknown],
            vec![CellStatus::Unknown, CellStatus::Unknown, CellStatus::Unknown, CellStatus::Unknown, CellStatus::Unknown, CellStatus::Unknown],
            vec![CellStatus::Unknown, CellStatus::Unknown, CellStatus::Tree, CellStatus::Unknown, CellStatus::Unknown, CellStatus::Unknown],
            vec![CellStatus::Unknown, CellStatus::Tree, CellStatus::Unknown, CellStatus::Unknown, CellStatus::Tree, CellStatus::Unknown]
        ];
        assert_eq!(test_board.size, expected_size);
        assert_eq!(test_board.row_counts, expected_row_counts);
        assert_eq!(test_board.column_counts, expected_col_counts);
        for (idx, row) in expected_board.into_iter().enumerate() {
            assert_eq!(test_board.board[idx], row);
        }
        assert_eq!(test_board.board.len(), 6);
    }

    #[test]
    fn test_only_space_for_tents() {
        let mut test_board = create_small_test_board();
        let result = test_board.only_space_for_tents();
        let expected_col_counts: Vec<Option<i8>> = vec![Some(1), None];
        let expected_row_counts: Vec<Option<i8>> = vec![None, None];
        let expected_size = 2;
        let expected_row1: Vec<CellStatus> = vec![CellStatus::Tree, CellStatus::Empty];
        let expected_row2: Vec<CellStatus> = vec![CellStatus::Tent, CellStatus::Empty];
        assert!(result);
        assert_eq!(test_board.column_counts, expected_col_counts);
        assert_eq!(test_board.row_counts, expected_row_counts);
        assert_eq!(test_board.size, expected_size);
        assert_eq!(test_board.board[0], expected_row1);
        assert_eq!(test_board.board[1], expected_row2);
    }

    fn create_test_board() -> Board {
        let mut board: Vec<Vec<CellStatus>> = Vec::new();
        let row1 = vec![CellStatus::Tent, CellStatus::Unknown, CellStatus::Unknown];
        let row2 = vec![CellStatus::Unknown, CellStatus::Tent, CellStatus::Unknown];
        let row3 = vec![CellStatus::Unknown, CellStatus::Unknown, CellStatus::Tent];
        board.push(row1);
        board.push(row2);
        board.push(row3);

        Board {
            size: 3,
            column_counts: vec![Some(1),None,Some(2)],
            row_counts: vec![Some(1),None,Some(2)],
            board,    
        }
    }

    fn create_small_test_board() -> Board {
        Board {
            size: 2,
            column_counts: vec![Some(1), None],
            row_counts: vec![None, None],
            board: vec![
                vec![CellStatus::Tree, CellStatus::Unknown],
                vec![CellStatus::Unknown, CellStatus::Unknown],
            ]
        }
    }
}