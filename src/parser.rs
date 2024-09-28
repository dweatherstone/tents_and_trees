use std::{fmt::Display, fs::File, io::Read};

use crate::{
    board::{Board, CellType},
    AppError,
};

#[derive(Debug)]
pub enum ParseErrorType {
    EmptyFile,
    MissingColumnClues,
    MissingRowClues,
    EmptyColumnClues,
    EmptyRowClues,
    InvalidClueLength(usize),
    InvalidRowLength(usize),
    InvalidBoardLength(usize),
    InvalidFormat,
}

impl Display for ParseErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseErrorType::EmptyFile => write!(f, "File is empty"),
            ParseErrorType::MissingColumnClues => write!(f, "Column clues are not present"),
            ParseErrorType::MissingRowClues => write!(f, "Row clues are not present"),
            ParseErrorType::EmptyColumnClues => write!(f, "The column clues cannot be read"),
            ParseErrorType::EmptyRowClues => write!(f, "The row clues cannot be read"),
            ParseErrorType::InvalidClueLength(len) => {
                write!(f, "There are the wrong number of clues: {}", len)
            }
            ParseErrorType::InvalidRowLength(row_num) => write!(
                f,
                "Board representation row {} does not have enough fields",
                row_num
            ),
            ParseErrorType::InvalidBoardLength(rows) => {
                write!(f, "Board representation only has {} rows", rows)
            }
            ParseErrorType::InvalidFormat => write!(f, "Invalid Format"),
        }
    }
}

pub fn get_board_from_file(file_path: &str) -> Result<Board, AppError> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    get_board_from_contents(&contents)
}

fn get_board_from_contents(contents: &str) -> Result<Board, AppError> {
    let (col_qty, row_qty) = parse_metadata(contents)?;
    let mut lines = contents.lines();
    let col_clues = get_clues(lines.next().unwrap(), col_qty)?;
    let row_clues = get_clues(lines.next().unwrap(), row_qty)?;
    let mut board = Vec::new();
    for row in lines {
        board.push(get_board_row(row, col_qty)?);
    }
    Ok(Board::new(board, col_clues, row_clues))
}

fn parse_metadata(contents: &str) -> Result<(usize, usize), AppError> {
    if contents.is_empty() {
        return Err(AppError::ParseError(ParseErrorType::EmptyFile));
    }

    let mut lines = contents.lines();
    let col_clues = lines
        .next()
        .ok_or(AppError::ParseError(ParseErrorType::MissingColumnClues))?;
    let col_clues_len = col_clues.split(",").count();
    if col_clues_len == 1 {
        return Err(AppError::ParseError(ParseErrorType::EmptyColumnClues));
    }
    let row_clues = lines
        .next()
        .ok_or(AppError::ParseError(ParseErrorType::MissingRowClues))?;
    let row_clues_len = row_clues.split(",").count();
    if row_clues_len == 1 {
        return Err(AppError::ParseError(ParseErrorType::EmptyRowClues));
    }
    let mut board_rows = 0usize;
    for (row_num, board_row) in lines.enumerate() {
        if board_row.split(",").count() != col_clues_len {
            return Err(AppError::ParseError(ParseErrorType::InvalidRowLength(
                row_num,
            )));
        }
        board_rows += 1;
    }
    if board_rows != row_clues_len {
        return Err(AppError::ParseError(ParseErrorType::InvalidBoardLength(
            board_rows,
        )));
    }

    Ok((col_clues_len, row_clues_len))
}

fn get_clues(line: &str, expected_len: usize) -> Result<Vec<Option<usize>>, AppError> {
    let clues = line.split(",").map(|val| val.trim());
    let mut result = Vec::new();
    for clue in clues {
        if clue.len() != 1 {
            return Err(AppError::ParseError(ParseErrorType::InvalidFormat));
        }
        if clue == "." || clue == "_" {
            result.push(None);
            continue;
        }
        if let Ok(clue_val) = clue.parse::<usize>() {
            result.push(Some(clue_val));
        } else {
            return Err(AppError::ParseError(ParseErrorType::InvalidFormat));
        }
    }
    if result.len() != expected_len {
        return Err(AppError::ParseError(ParseErrorType::InvalidRowLength(
            result.len(),
        )));
    }
    Ok(result)
}

fn get_board_row(row_str: &str, expected_len: usize) -> Result<Vec<CellType>, AppError> {
    let row = row_str.split(",").map(|val| val.trim());
    let mut result = Vec::new();
    for value in row {
        if value.len() != 1 {
            return Err(AppError::ParseError(ParseErrorType::InvalidFormat));
        }
        let cell_type = match value {
            "." | "_" => CellType::Unknown,
            "t" | "T" => CellType::Tree,
            "x" | "X" => CellType::Tent,
            "u" | "U" => CellType::Unknown,
            _ => return Err(AppError::ParseError(ParseErrorType::InvalidFormat)),
        };
        result.push(cell_type)
    }
    if result.len() != expected_len {
        return Err(AppError::ParseError(ParseErrorType::InvalidRowLength(
            result.len(),
        )));
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Board;

    #[test]
    fn file_not_found() {
        let result = get_board_from_file("non_existent_file.txt");

        // Assert that the result is an Err variant and matches our IoError case
        match result {
            Err(AppError::IoError(ref e)) => {
                assert_eq!(e.kind(), std::io::ErrorKind::NotFound);
            }
            _ => panic!("Expected IoError::NotFound, but got {:?}", result),
        }
    }

    #[test]
    fn empty_file_content() {
        // Simulate an empty content case (we don't have to actually write a file here)
        let result = parse_metadata("");
        assert!(matches!(
            result,
            Err(AppError::ParseError(ParseErrorType::EmptyFile))
        ));
    }

    #[test]
    fn only_1_line() {
        let result = parse_metadata("1,2,3");
        assert!(matches!(
            result,
            Err(AppError::ParseError(ParseErrorType::MissingRowClues))
        ));
    }

    #[test]
    fn only_2_lines() {
        let result = parse_metadata("1,2,3\n4,5,6");
        assert!(matches!(
            result,
            Err(AppError::ParseError(ParseErrorType::InvalidBoardLength(0)))
        ));
    }

    #[test]
    fn empty_col_clues() {
        let failed_contents = "
2,3,4
.,.,T
T,.,.
.,.,.";
        let result = parse_metadata(failed_contents);
        assert!(matches!(
            result,
            Err(AppError::ParseError(ParseErrorType::EmptyColumnClues))
        ));
    }

    #[test]
    fn empty_row_clues() {
        let failed_contents = "1,2,3

.,.,T
T,.,.
.,.,.";
        let result = parse_metadata(failed_contents);
        assert!(matches!(
            result,
            Err(AppError::ParseError(ParseErrorType::EmptyRowClues))
        ));
    }

    #[test]
    fn incorrect_board_row_len() {
        let failed_contents = "1,2,3
2,3,4
.,.,T
T,.
.,.,.";
        let result = parse_metadata(failed_contents);
        assert!(matches!(
            result,
            Err(AppError::ParseError(ParseErrorType::InvalidRowLength(1)))
        ));
    }

    #[test]
    fn incorrect_board_len() {
        let failed_contents = "1,2,3
2,3,4
.,.,T
T,.,.";
        let result = parse_metadata(failed_contents);
        assert!(matches!(
            result,
            Err(AppError::ParseError(ParseErrorType::InvalidBoardLength(2)))
        ));
    }

    #[test]
    fn successful_parse() {
        let successful_contents = "1,.,3
0,3,.
.,.,T
T,.,.
.,.,.";
        let metadata_result = parse_metadata(successful_contents);
        assert!(metadata_result.is_ok());

        let expected_board = vec![
            vec![CellType::Unknown, CellType::Unknown, CellType::Tree],
            vec![CellType::Tree, CellType::Unknown, CellType::Unknown],
            vec![CellType::Unknown, CellType::Unknown, CellType::Unknown],
        ];
        let expected_col_clues = vec![Some(1), None, Some(3)];
        let expected_row_clues = vec![Some(0), Some(3), None];
        let expected_board = Board::new(expected_board, expected_col_clues, expected_row_clues);
        let result = get_board_from_contents(successful_contents);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, expected_board);
    }
}
