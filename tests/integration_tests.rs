use buscaminas::board::Board;
use std::fs::read_to_string;

#[test]
fn integration_test_with_empty_board() {
    let board_data: String =
        read_to_string("./tablero_vacio.txt").expect("Could not open empty board file.");
    let board: Board = Board::new(board_data.as_bytes());
    assert_eq!(board.mine_count(), ".....\n.....\n.....\n.....\n.....\n");
}

#[test]
fn integration_test_with_normal_board_1() {
    let board_data: String =
        read_to_string("./tablero_normal_1.txt").expect("Could not open normal board 1 file.");
    let board: Board = Board::new(board_data.as_bytes());
    assert_eq!(board.mine_count(), "1*3*1\n13*31\n.2*2.\n.111.\n");
}

#[test]
fn integration_test_with_normal_board_2() {
    let board_data: String =
        read_to_string("./tablero_normal_2.txt").expect("Could not open normal {} 2 file.");
    let board: Board = Board::new(board_data.as_bytes());
    assert_eq!(board.mine_count(), ".2*42\n13***\n*334*\n2*111\n");
}
