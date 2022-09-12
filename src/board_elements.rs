#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]

/// Representa el contenido del tablero. No pretende ser usado por fuera del tablero.
/// Es de uso privado del mismo para obtener una mayor organizacion.
pub enum BoardElements {
    /// Representa al salto de linea o '\n'.
    Newline = b'\n',
    /// Representa a una mina o '*'.
    Mine = b'*',
    /// Representa a un espacio vacio o '.'.
    Empty = b'.',
    /// Representa a un error o 'E'. Este ultimo no deberia aparecer nunca ya que solo ocurre cuando hay algun tipo de error.
    Error = b'E',
}

/// Permite transformar caracteres en codigo ASCII a su representacion en el tablero.
/// Se usa para comparar la entrada en codigo ASCII y poder crear el tablero correctamente.
impl From<u8> for BoardElements {
    fn from(item: u8) -> BoardElements {
        match item {
            10 => BoardElements::Newline,
            42 => BoardElements::Mine,
            46 => BoardElements::Empty,
            _ => BoardElements::Error,
        }
    }
}


/// Permite transformar cada elemento del tablero a char para poder imprimir el tablero
impl From<char> for BoardElements {
    fn from(item: char) -> BoardElements {
        BoardElements::from(item as u8)
    }
}

