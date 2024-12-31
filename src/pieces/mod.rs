mod pawn;

pub struct Piece {
    kind: Kind,
    color: Color,
    position: Position,
    valid_moves: Vec<ValidMoves>,
    //menaces: Vec<Piece>,
}

pub enum Kind {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}

pub enum Color {
    Black,
    White,
}

pub struct Position {
    x: u32,
    y: u32,
}

pub type ValidMoves = cgmath::Vector2<i32>;

pub trait Move {
    fn make_a_move(&self, new_x: u32, new_y: u32) -> Position;
    fn valid_position(&self, new_x: u32, new_y: u32) -> Position;
}
