use std::{fmt::Display, io};

// use backtracker::Config;
use board::BoardErrorType;
use parser::ParseErrorType;
use solver::TentsAndTreesConfig;

mod backtracker;
pub mod board;
pub mod parser;
pub mod solver;

#[derive(Debug)]
pub enum AppError {
    IoError(io::Error),
    ParseError(ParseErrorType),
    BoardError(BoardErrorType),
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::IoError(e) => write!(f, "I/O error: {}", e),
            AppError::ParseError(e) => write!(f, "{}", format_args!("Parse error: {}", e)),
            AppError::BoardError(e) => write!(f, "{}", format_args!("Board error: {}", e)),
        }
    }
}

impl std::error::Error for AppError {}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError::IoError(error)
    }
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), AppError> {
    // TODO: Also seems to be duplicating possibilities.
    let file_path = "examples/example1.txt";
    // let board = parser::get_board_from_file("examples/example_fail.txt")?;
    let mut board = parser::get_board_from_file(file_path)?;
    println!("Initial board from file '{}':", file_path);
    println!("{}", board);

    // TODO: Update the board with the easy wins - 0 clues, no trees to North, South, East or West
    board.set_mandatory_empty();

    println!("Board after set_mandatory_empty:");
    println!("{}", board);

    let config = TentsAndTreesConfig::new(&board);

    // let possible_configs = config.successors();
    // println!(
    //     "Possible Configs, first step: {} possibilities",
    //     possible_configs.len()
    // );
    // for pc in possible_configs {
    //     println!("{}", pc.board)
    // }

    // Solve the puzzle
    println!(
        "{}",
        match backtracker::solve(config) {
            Some(solution) => format!("SOLUTION FOUND:\n{}", solution),
            None => "No solution found".to_string(),
        }
    );

    Ok(())
}
