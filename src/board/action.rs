use crate::board::Board;
use crate::pieces::{Kind, Piece, Position};

pub fn get_piece(board: &Board, x: usize, y: usize) -> Option<&Piece> {
    let piece = board[y - 1][x - 1].as_ref();

    if piece.is_some() {
        Some(piece?)
    } else {
        None
    }
}

pub fn move_piece(
    board: &mut Board,
    from: (usize, usize),
    to: (usize, usize),
) -> Result<(), String> {
    if let Some(piece) = get_piece(board, from.0, from.1) {
        if piece
            .valid_moves
            .iter()
            .any(|m| m.x == to.0 as i8 && m.y == to.1 as i8)
        {
            if let Some(mut moving_piece) = board[from.1 - 1][from.0 - 1].take() {
                moving_piece.position = Position {
                    x: to.0 as u8,
                    y: to.1 as u8,
                };

                match moving_piece.kind {
                    Kind::Pawn => moving_piece.update_pawn_moves(),
                    _ => moving_piece.updated_new_moves(),
                }

                board[to.1 - 1][to.0 - 1] = Some(moving_piece);

                Ok(())
            } else {
                Err("Unexpected error: piece disappeared".to_string())
            }
        } else {
            Err("Invalid move".to_string())
        }
    } else {
        Err("No piece at the source position".to_string())
    }
}
