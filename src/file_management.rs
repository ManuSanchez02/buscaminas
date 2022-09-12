use std::env;
use std::fs::read_to_string;
use std::io::{Error, ErrorKind, Result};

/// Lee el archivo pasado como parametro y devuelve su contenido envuelto en un Result.
///
/// # Error
/// Devuelve error si no se paso un argumento a la hora de ejecutar el programa. Tambien
/// devuelve error en caso de lectura erronea del archivo o si el archivo esta vacio.
///
/// # Ejemplo
/// Si el programa fuera ejecutado como: cargo run ./tablero.txt, la funcion lee el archivo
/// ./tablero.txt y devuelve su contenido.
pub fn read_file_from_first_argument() -> Result<String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Error while trying to read the file: Expected 1 argument.",
        ));
    }

    let data = match read_to_string(&args[1]) {
        Err(error) => return Err(error),
        Ok(data) => data,
    };

    if data.is_empty() {
        return Err(Error::new(ErrorKind::InvalidInput, "File is empty."));
    }

    Ok(data)
}
