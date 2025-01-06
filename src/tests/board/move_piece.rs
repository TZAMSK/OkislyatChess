#[cfg(test)]
mod tests {
    use crate::board::action::{get_piece, move_piece};
    use crate::board::init_board;
    use crate::pieces::{Color, Kind, Piece, Position, ValidMove};
    use crate::tests::board::count_pieces_on_board;

    #[test]
    fn moved_white_pawn_from_1_2_to_1_3_isnt_on_original_position_but_on_new() {
        let mut board = init_board();

        move_piece(&mut board, (1, 2), (1, 3)).expect("a");

        assert_eq!(get_piece(&board, 1, 2), None);
        assert_eq!(
            get_piece(&board, 1, 3),
            Some(&Piece {
                kind: Kind::Pawn,
                color: Color::White,
                position: Position { x: 1, y: 3 },
                valid_moves: vec![
                    ValidMove { x: 0, y: 4 },
                    ValidMove { x: 1, y: 4 },
                    ValidMove { x: 2, y: 4 },
                ]
            })
        );
    }

    #[test]
    fn moving_a_piece_doesnt_leave_a_copy_behind_and_the_count_of_pieces_stays_32() {
        let mut board = init_board();

        assert_eq!(count_pieces_on_board(&board, Some(())), 32);

        move_piece(&mut board, (1, 2), (1, 3)).expect("a");

        assert_eq!(count_pieces_on_board(&board, Some(())), 32);
    }

    #[test]
    fn moving_a_piece_doesnt_leave_a_copy_behind_and_the_count_of_empty_cells_stays_32() {
        let mut board = init_board();

        assert_eq!(count_pieces_on_board(&board, None::<()>), 32);

        move_piece(&mut board, (1, 2), (1, 3)).expect("a");
        assert_eq!(count_pieces_on_board(&board, None::<()>), 32);
    }
}
