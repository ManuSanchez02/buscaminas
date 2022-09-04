use std::env;
use std::fs::read_to_string;
use std::io::{Error, ErrorKind, Result};
mod board;
use crate::board::Board;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Error while trying to read the file: Expected 1 argument.",
        ));
    }

    let data = read_to_string(&args[1]).expect("Error while trying to read the file.");
    if data.is_empty() {
        return Err(Error::new(ErrorKind::InvalidInput, "File is empty."));
    }

    let board = Board::new(data.as_bytes());
    board.print_mine_count();

    Ok(())
}
