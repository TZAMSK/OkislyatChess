mod get_piece;
mod move_piece;
mod pieces_count;
mod pieces_initial_coords;

use crate::board::Board;
use crate::pieces::{Color, Kind, Piece, Position};

fn count_piece_by_type<'a>(
    board: &'a Board,
    kind: Kind,
    color: Option<Color>,
) -> Vec<&'a Option<Piece>> {
    let type_piece_count: Vec<_> = board
        .iter()
        .flat_map(|row| {
            row.iter().filter(|cell| match cell {
                Some(piece) => {
                    piece.kind == kind && (color.is_none() || piece.color == color.clone().unwrap())
                }
                _ => false,
            })
        })
        .collect();

    type_piece_count
}

fn get_piece_coords(board: &Board, kind: Kind, color: Option<Color>) -> Vec<Position> {
    count_piece_by_type(board, kind, color)
        .into_iter()
        .filter_map(|opt_piece| opt_piece.as_ref().map(|piece| piece.position.clone()))
        .collect()
}

fn count_pieces_on_board<'a, T>(board: &'a Board, cell_type: Option<T>) -> usize {
    board
        .iter()
        .flat_map(|row| {
            row.iter().filter(|piece| {
                if cell_type.is_some() {
                    piece.is_some()
                } else {
                    piece.is_none()
                }
            })
        })
        .count()
}
