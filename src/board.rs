use std::char;

const NEWLINE_CHAR: char = '\n';
const MINE_CHAR: char = '*';
const EMPTY_CHAR: char = '.';
const ERROR_CHAR: char = 'E';

const NEWLINE: u8 = NEWLINE_CHAR as u8;
const MINE: u8 = MINE_CHAR as u8;
const EMPTY: u8 = EMPTY_CHAR as u8;

/// Representa a un tablero del juego Buscaminas.
pub struct Board {
    /// La matriz data contiene al tablero del buscaminas en formato u8.
    data: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

/// Principales funciones del `struct Board`.
impl Board {
    /// Crea un nuevo Board a partir de un slice de u8.
    ///
    /// # Argumentos
    ///
    /// * `data` - Es un slice de u8 que permite inicializar la matriz `Board.data`.
    ///            Si dentro del slice data hay un `\n`, se toma como si fuera una nueva fila en la matriz.
    ///            Debe haber la misma cantidad de elementos por fila para garantizar el correcto funcionamiento del Board.
    ///            Finalmente, debe finalizar con un [`NEWLINE`](NEWLINE).
    ///
    /// # Elementos dentro del slice
    ///
    /// * [`MINE`](MINE)
    /// * [`NEWLINE`](NEWLINE)
    ///
    /// Todos los otros caracteres son considerados espacios vacios.
    ///
    /// # Ejemplo
    /// ```rust
    /// mod board;
    /// use crate::board::Board;
    /// fn main() -> Result<()> {
    ///     // Notar como data tiene 5 elementos entre cada `\n` y debe finalizar con el mismo.
    ///     let data = ".*.*.\n..*..\n..*..\n.....\n";
    ///     let board = Board::new(data.as_bytes());
    /// }
    /// ```
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
            } else if *char == MINE{
                row.push(*char);
            } else {
                row.push(EMPTY);
            }
        }
        Board {
            data: matrix,
            width,
            height,
        }
    }

    /// Imprime el tablero agregandole, en los espacios vacios, la cantidad de minas en los casilleros vecinos. Si ocurriera algun tipo de error,
    /// escribe `ERROR_CHAR` en el casillero del error.
    ///
    /// Internamente, imprime el `String` generado usando la funcion [`Board::mine_count(&self)`](Board::mine_count), ver la misma para mas detalles.
    ///
    /// # Ejemplo
    /// Si `Board.data` contiene la siguiente matriz:
    /// ```
    /// [
    ///    [46, 42, 46, 42, 46]
    ///    [46, 46, 42, 46, 46]
    ///    [46, 46, 42, 46, 46]
    ///    [46, 46, 46, 46, 46]
    /// ]
    /// ```
    /// Al ejecutar esta funcion, obtendremos a traves de stdout lo siguiente:
    /// ```text
    /// 1*3*1
    /// 13*31
    /// .2*2.
    /// .111.
    /// ```
    pub fn print_mine_count(&self) {
        println!("{}", self.mine_count());
    }

    /// Devuelve un `String` con forma de matriz que contiene las minas y, en los casilleros vacios, la cantidad de minas que rodean a cada casillero.
    /// Este `String` contiene `\n` que delimitan las filas de la matriz. En consecuencia, al ser impreso, toma forma de matriz.
    ///
    /// # Caracteres del String
    /// * [`MINE_CHAR`](MINE_CHAR): Representa las minas.
    /// * [`EMPTY_CHAR`](EMPTY_CHAR): Representa los espacios vacios.
    /// * [`NEWLINE_CHAR`](NEWLINE_CHAR): Marca los saltos de linea.
    /// * [`ERROR_CHAR`](ERROR_CHAR): Solo es escrito en caso de error.
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

    #[doc(hidden)]
    /// Dada una posicion de un casillero, devuelve la cantidad de minas que rodean al mismo.
    /// # Argumentos
    ///
    /// * `row` - Es un `usize` que representa la fila del casillero al que se le quieren contar las minas vecinas.
    ///
    /// * `column` - Es un `usize` que representa la columna del casillero al que se le quieren contar las minas vecinas.
    fn count_surrounding_mines(&self, row: usize, column: usize) -> u8 {
        let mut surrounding_mines: u8 = 0;
        let board_width = self.width;
        let board_height = self.height;
        let row_aux: isize = row as isize;
        let column_aux: isize = column as isize;

        let vertical_bounds = self.get_bounds(board_height, row);
        let horizontal_bounds = self.get_bounds(board_width, column);

        for i in vertical_bounds.0..(vertical_bounds.1 + 1) {
            for j in horizontal_bounds.0..(horizontal_bounds.1 + 1) {
                if self.data[(row_aux + i) as usize][(column_aux + j) as usize] == MINE
                    && !(i == 0 && j == 0)
                {
                    surrounding_mines += 1;
                }
            }
        }
        surrounding_mines
    }

    #[doc(hidden)]
    /// Devuelve los limites para contar la cantidad de minas vecinas.
    /// # Argumentos
    ///
    /// * `board_size_in_axis` - Es un `usize` que representa el tamaño del tablero en cierto eje (vertical u horizontal).
    ///
    /// * `column` - Es un `usize` que representa la posicion del casillero en ese eje (vertical u horizontal).
    ///
    /// Ambos argumentos deben referirse al mismo eje para garantizar el correcto funcionamiento.
    fn get_bounds(&self, board_size_in_axis: usize, position_in_axis: usize) -> (isize, isize) {
        let mut bounds: (isize, isize) = (-1, 1);
        if position_in_axis == 0 {
            bounds.0 = 0;
        } else if position_in_axis == (board_size_in_axis - 1) {
            bounds.1 = 0;
        }

        bounds
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    use crate::board::{EMPTY, MINE};

    use super::Board;

    #[test]
    fn board_initializes_correctly() {
        let data = ".*...\n..**.\n..*..\n....*\n";
        let board = Board::new(data.as_bytes());
        let matrix = vec![
            vec![EMPTY, MINE, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, MINE, MINE, EMPTY],
            vec![EMPTY, EMPTY, MINE, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, MINE],
            ];

        assert_eq!(board.height, 4);
        assert_eq!(board.width, 5);
        assert_eq!(board.data, matrix);
    }

    #[test]
    fn count_mines_returns_correct_number_of_mines() {
        let data = "..*..\n..***\n*...*\n.*...\n";
        let board = Board::new(data.as_bytes());
        let expected_result = ".2*42\n13***\n*334*\n2*111\n";
        
        assert_eq!(board.mine_count(), expected_result);
    }

    #[test]
    fn count_mines_on_empty_board_returns_empty_board() {
        let data = ".....\n.....\n.....\n.....\n";
        let board = Board::new(data.as_bytes());
        let expected_result = ".....\n.....\n.....\n.....\n";
        
        assert_eq!(board.mine_count(), expected_result);
    }

    #[test]
    fn count_mines_on_full_board_returns_full_board() {
        let data = "*****\n*****\n*****\n*****\n";
        let board = Board::new(data.as_bytes());
        let expected_result = "*****\n*****\n*****\n*****\n";
        
        assert_eq!(board.mine_count(), expected_result);
    }
}
