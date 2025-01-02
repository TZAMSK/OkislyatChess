#[cfg(test)]
mod tests {
    use crate::board::action::move_piece;
    use crate::board::init_board;
    use crate::pieces::{Color, Kind, Piece, Position, ValidMove};

    #[test]
    fn board_has_64_cells() {
        let mut board = init_board();

        move_piece(&mut board, (1, 2), (1, 3)).expect("a");

        assert_eq!(board.len(), 64);
    }
}
