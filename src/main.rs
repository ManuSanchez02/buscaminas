use std::env;
use std::fs::read_to_string;
use std::io::{Error, ErrorKind, Result};
mod board;
use crate::board::Board;

/// Imprime por pantalla el tablero pasado como argumento de linea de comandos, pero con un conteo de minas vecinas por casillero.
///
/// # Argumentos
///
/// * `path_al_tablero` - Este argumento se recibe mediante linea de comandos. Indica la ruta del archivo a ser leido para realizar el conteo de minas.
///
/// # Error
/// El programa solo puede recibir 1 argumento. En caso contrario, se arrojara un error indicando que se esperaba 1 argumento.
/// Si el tablero no se puede encontrar, el programa devuelve un error con el mensaje de que no existe tal archivo.
fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Error while trying to read the file: Expected 1 argument.",
        ));
    }

    let data = read_to_string(&args[1]);
    match data {
        Err(error) => Err(error),
        Ok(data) => {
            if data.is_empty() {
                return Err(Error::new(ErrorKind::InvalidInput, "File is empty."));
            }

            let board = Board::new(data.as_bytes());
            board.print_mine_count();

            Ok(())
        }
    }
}
