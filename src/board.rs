use std::char;

const NEWLINE: u8 = 10;
const MINE: u8 = 42;
const MINE_CHAR: char = '*';
const EMPTY_CHAR: char = '.';
const NEWLINE_CHAR: char = '\n';
const ERROR_CHAR: char = 'E';

pub struct Board {
    data: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl Board {
    pub fn new(data: &[u8]) -> Self {
        let mut width: usize = 0;
        let mut matrix: Vec<Vec<u8>> = vec![];
        while data[width] != NEWLINE {
            width += 1;
        }
        let width_con_newline = width + 1;
        let height: usize = data.len() / (width_con_newline);
        let mut row: Vec<u8> = vec![];

        for char in data {
            if *char == NEWLINE {
                matrix.push(row);
                row = vec![];
            } else {
                row.push(*char);
            }
        }
        matrix.push(row);
        Board {
            data: matrix,
            width,
            height,
        }
    }

    pub fn print_mine_count(&self) {
        println!("{}", self.mine_count());
    }

    fn mine_count(&self) -> String {
        let mut resultado: String = String::new();
        for i in 0..self.height {
            for j in 0..self.width {
                if self.data[i][j] == MINE {
                    resultado.push(MINE_CHAR);
                } else {
                    match self.count_surrounding_mines(i, j) {
                        0 => resultado.push(EMPTY_CHAR),
                        count => {
                            let count_char = char::from_digit(count as u32, 10);
                            match count_char {
                                None => resultado.push(ERROR_CHAR),
                                Some(c) => resultado.push(c),
                            }
                        }
                    }
                }
            }
            resultado.push(NEWLINE_CHAR);
        }
        resultado
    }

    fn count_surrounding_mines(&self, row: usize, column: usize) -> u8 {
        let mut surrounding_mines: u8 = 0;
        let board_width = self.width;
        let board_height = self.height;

        let vertical_bounds = self.get_bounds(board_height, row);
        let horizontal_bounds = self.get_bounds(board_width, column);

        for i in vertical_bounds.0..vertical_bounds.1 {
            for j in horizontal_bounds.0..horizontal_bounds.1 {
                if self.data[row + i - 1][column + j - 1] == MINE && !(i == 1 && j == 1) {
                    surrounding_mines += 1;
                }
            }
        }
        surrounding_mines
    }

    fn get_bounds(&self, board_size_in_axis: usize, position_in_axis: usize) -> (usize, usize) {
        let mut bounds: (usize, usize) = (0, 3);
        if position_in_axis == 0 {
            bounds.0 = 1;
        } else if position_in_axis == (board_size_in_axis - 1) {
            bounds.1 = 2;
        }

        bounds
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test() {}
}
