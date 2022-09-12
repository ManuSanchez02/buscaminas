use buscaminas::board::Board;
use buscaminas::file_management::read_file_from_first_argument;
use std::io::Result;

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
    let data: String = match read_file_from_first_argument() {
        Err(error) => return Err(error),
        Ok(data) => data,
    };

    let board: Board = Board::new(data.as_bytes());
    board.print_mine_count();

    Ok(())
}
