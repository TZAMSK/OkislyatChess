pub mod action;

use crate::pieces::{Color, Kind, Piece, Position};

pub type Board = [[Option<Piece>; 8]; 8];

pub fn init_board() -> Board {
    [
        [
            create_piece(Kind::Rook, Color::White, 1, 1),
            create_piece(Kind::Knight, Color::White, 2, 1),
            create_piece(Kind::Bishop, Color::White, 3, 1),
            create_piece(Kind::Queen, Color::White, 4, 1),
            create_piece(Kind::King, Color::White, 5, 1),
            create_piece(Kind::Bishop, Color::White, 6, 1),
            create_piece(Kind::Knight, Color::White, 7, 1),
            create_piece(Kind::Rook, Color::White, 8, 1),
        ],
        [
            create_piece(Kind::Pawn, Color::White, 1, 2),
            create_piece(Kind::Pawn, Color::White, 2, 2),
            create_piece(Kind::Pawn, Color::White, 3, 2),
            create_piece(Kind::Pawn, Color::White, 4, 2),
            create_piece(Kind::Pawn, Color::White, 5, 2),
            create_piece(Kind::Pawn, Color::White, 6, 2),
            create_piece(Kind::Pawn, Color::White, 7, 2),
            create_piece(Kind::Pawn, Color::White, 8, 2),
        ],
        [None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None],
        [
            create_piece(Kind::Pawn, Color::Black, 1, 7),
            create_piece(Kind::Pawn, Color::Black, 2, 7),
            create_piece(Kind::Pawn, Color::Black, 3, 7),
            create_piece(Kind::Pawn, Color::Black, 4, 7),
            create_piece(Kind::Pawn, Color::Black, 5, 7),
            create_piece(Kind::Pawn, Color::Black, 6, 7),
            create_piece(Kind::Pawn, Color::Black, 7, 7),
            create_piece(Kind::Pawn, Color::Black, 8, 7),
        ],
        [
            create_piece(Kind::Rook, Color::Black, 1, 8),
            create_piece(Kind::Knight, Color::Black, 2, 8),
            create_piece(Kind::Bishop, Color::Black, 3, 8),
            create_piece(Kind::Queen, Color::Black, 4, 8),
            create_piece(Kind::King, Color::Black, 5, 8),
            create_piece(Kind::Bishop, Color::Black, 6, 8),
            create_piece(Kind::Knight, Color::Black, 7, 8),
            create_piece(Kind::Rook, Color::Black, 8, 8),
        ],
    ]
}

pub fn create_piece(kind: Kind, color: Color, x: u8, y: u8) -> Option<Piece> {
    Some(Piece::new(kind, color, Position { x, y }))
}
