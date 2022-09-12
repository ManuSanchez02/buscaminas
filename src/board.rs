use std::char;

pub const NEWLINE_CHAR: char = '\n';
pub const MINE_CHAR: char = '*';
pub const EMPTY_CHAR: char = '.';
pub const ERROR_CHAR: char = 'E';

pub const NEWLINE: u8 = NEWLINE_CHAR as u8;
pub const MINE: u8 = MINE_CHAR as u8;
pub const EMPTY: u8 = EMPTY_CHAR as u8;

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
    ///            Finalmente, debe finalizar con un [`NEWLINE`](NEWLINE). Asimismo, data no puede estar vacio para asegurarse
    ///            que todo funcione correctamente (es una precondicion).
    ///
    /// # Elementos dentro del slice
    ///
    /// * [`MINE`](MINE)
    /// * [`NEWLINE`](NEWLINE)
    /// * [`EMPTY`](EMPTY) (Cualquier caracter distinto de los 2 anteriores sera considerado como EMPTY)
    ///
    /// Todos los otros caracteres son considerados espacios vacios.
    ///
    /// # Ejemplo
    /// ```rust
    /// use buscaminas::board::Board;
    /// fn main() {
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
        let width_con_newline: usize = width + 1;
        let height: usize = data.len() / (width_con_newline);
        let mut row: Vec<u8> = vec![];

        for char in data {
            match *char {
                NEWLINE => {
                    matrix.push(row);
                    row = vec![];
                }
                MINE => row.push(MINE),
                _ => row.push(EMPTY),
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
    /// let board_data = [
    ///                     [46, 42, 46, 42, 46],
    ///                     [46, 46, 42, 46, 46],
    ///                     [46, 46, 42, 46, 46],
    ///                     [46, 46, 46, 46, 46],
    ///                  ];
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
    /// 
    /// # Ejemplo
    /// Si `Board.data` contiene la siguiente matriz:
    /// ```
    /// let board_data = [
    ///                     [46, 42, 46, 42, 46],
    ///                     [46, 46, 42, 46, 46],
    ///                     [46, 46, 42, 46, 46],
    ///                     [46, 46, 46, 46, 46],
    ///                  ];
    /// ```
    /// Al ejecutar la funcion, obtendremos:
    /// ```rust
    /// use buscaminas::board::Board;
    /// 
    /// fn main() {
    ///     let data: &str = "..*..\n..***\n*...*\n.*...\n";
    ///     let board: Board = Board::new(data.as_bytes());
    ///     let board_with_mine_count = board.mine_count(); // Devuelve ".2*42\n13***\n*334*\n2*111\n"
    /// }
    /// ```
    pub fn mine_count(&self) -> String {
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
        let row_aux: isize = row as isize;
        let column_aux: isize = column as isize;

        let vertical_bounds: (isize, isize) = self.get_bounds(true, row);
        let horizontal_bounds: (isize, isize) = self.get_bounds(false, column);

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
    /// * `vertical` - Indica el eje respecto al que se quiere saber los limites (vertical u horizontal).
    ///                Si es true indica el eje vertical y si es false, el horizontal
    ///
    /// * `column` - Es un `usize` que representa la posicion del casillero en ese eje (vertical u horizontal).
    ///
    /// Ambos argumentos deben referirse al mismo eje para garantizar el correcto funcionamiento.
    fn get_bounds(&self, vertical: bool, position_in_axis: usize) -> (isize, isize) {
        let board_size_in_axis = if vertical { self.height } else { self.width };

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
        let data: &str = ".*...\n..**.\n..*..\n....*\n";
        let board: Board = Board::new(data.as_bytes());
        let matrix: Vec<Vec<u8>> = vec![
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
    fn get_bounds_returns_correct_vertical_bounds_on_borders() {
        let data: &str = "..*..\n..***\n*...*\n.*...\n";
        let board: Board = Board::new(data.as_bytes());

        assert_eq!(board.get_bounds(true, 0), (0, 1));
        assert_eq!(board.get_bounds(true, board.height - 1), (-1, 0));
    }

    #[test]
    fn get_bounds_returns_correct_horizontal_bounds_on_borders() {
        let data: &str = "..*..\n..***\n*...*\n.*...\n";
        let board: Board = Board::new(data.as_bytes());

        assert_eq!(board.get_bounds(false, 0), (0, 1));
        assert_eq!(board.get_bounds(false, board.width - 1), (-1, 0));
    }

    #[test]
    fn get_bounds_returns_correct_bounds_on_middle() {
        let data: &str = "..*..\n..***\n*...*\n.*...\n";
        let board: Board = Board::new(data.as_bytes());

        assert_eq!(board.get_bounds(false, 2), (-1, 1));
        assert_eq!(board.get_bounds(true, 2), (-1, 1));
    }

    #[test]
    fn counting_surrounding_mines_for_edge_position_returns_correct_value() {
        let data: &str = ".**..\n*.***\n*...*\n.*.*.\n";
        let board: Board = Board::new(data.as_bytes());

        assert_eq!(board.count_surrounding_mines(0, 0), 2);
    }

    #[test]
    fn counting_surrounding_mines_for_border_position_returns_correct_value() {
        let data: &str = ".**..\n*.***\n.*..*\n.*.*.\n";
        let board: Board = Board::new(data.as_bytes());

        assert_eq!(board.count_surrounding_mines(2, 0), 3);
    }

    #[test]
    fn counting_surrounding_mines_for_middle_position_returns_correct_value() {
        let data: &str = ".**..\n*.***\n.*..*\n.*.*.\n";
        let board: Board = Board::new(data.as_bytes());

        assert_eq!(board.count_surrounding_mines(2, 3), 5);
    }

    #[test]
    fn count_mines_returns_correct_number_of_mines() {
        let data: &str = "..*..\n..***\n*...*\n.*...\n";
        let board: Board = Board::new(data.as_bytes());
        let expected_result: &str = ".2*42\n13***\n*334*\n2*111\n";

        assert_eq!(board.mine_count(), expected_result);
    }

    #[test]
    fn count_mines_on_empty_board_returns_empty_board() {
        let data: &str = ".....\n.....\n.....\n.....\n";
        let board: Board = Board::new(data.as_bytes());
        let expected_result: &str = ".....\n.....\n.....\n.....\n";

        assert_eq!(board.mine_count(), expected_result);
    }

    #[test]
    fn count_mines_on_full_board_returns_full_board() {
        let data: &str = "*****\n*****\n*****\n*****\n";
        let board: Board = Board::new(data.as_bytes());
        let expected_result: &str = "*****\n*****\n*****\n*****\n";

        assert_eq!(board.mine_count(), expected_result);
    }
}
