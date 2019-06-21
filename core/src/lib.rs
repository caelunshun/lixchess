#![feature(test)]
#[cfg(test)]
extern crate test;

#[macro_use]
extern crate derive_new;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum PieceType {
    Pawn,
    Bishop,
    Knight,
    Rook,
    King,
    Queen,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, new)]
pub struct Piece {
    ty: PieceType,
    color: Color,
}

impl Piece {
    pub fn ty(&self) -> PieceType {
        self.ty
    }

    pub fn color(&self) -> Color {
        self.color
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Color {
    Black,
    White,
}

impl Color {
    pub fn opposite(self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

pub type Pieces = [[Option<Piece>; 8]; 8];

#[derive(Debug, Clone)]
pub struct Chessboard {
    pieces: Pieces,
}

impl Chessboard {
    /// Returns a chessboard with
    /// the standard starting position.
    pub fn new() -> Self {
        // Start with empty board
        let mut board = Self {
            pieces: [[None; 8]; 8],
        };

        // Initialize pieces
        for x in 0..8 {
            board.set_piece_at(Position(x, 1), Piece::new(PieceType::Pawn, Color::White));
        }

        board.set_piece_at(Position(0, 0), Piece::new(PieceType::Rook, Color::White));
        board.set_piece_at(Position(7, 0), Piece::new(PieceType::Rook, Color::White));

        board.set_piece_at(Position(1, 0), Piece::new(PieceType::Knight, Color::White));
        board.set_piece_at(Position(6, 0), Piece::new(PieceType::Knight, Color::White));

        board.set_piece_at(Position(2, 0), Piece::new(PieceType::Bishop, Color::White));
        board.set_piece_at(Position(5, 0), Piece::new(PieceType::Bishop, Color::White));

        board.set_piece_at(Position(4, 0), Piece::new(PieceType::King, Color::White));
        board.set_piece_at(Position(3, 0), Piece::new(PieceType::Queen, Color::White));

        for x in 0..8 {
            board.set_piece_at(Position(x, 6), Piece::new(PieceType::Pawn, Color::Black));
        }

        board.set_piece_at(Position(7, 7), Piece::new(PieceType::Rook, Color::Black));
        board.set_piece_at(Position(0, 7), Piece::new(PieceType::Rook, Color::Black));

        board.set_piece_at(Position(1, 7), Piece::new(PieceType::Knight, Color::Black));
        board.set_piece_at(Position(6, 7), Piece::new(PieceType::Knight, Color::Black));

        board.set_piece_at(Position(2, 7), Piece::new(PieceType::Bishop, Color::Black));
        board.set_piece_at(Position(5, 7), Piece::new(PieceType::Bishop, Color::Black));

        board.set_piece_at(Position(3, 7), Piece::new(PieceType::King, Color::Black));
        board.set_piece_at(Position(4, 7), Piece::new(PieceType::Queen, Color::Black));

        board
    }

    pub fn set_piece_at(&mut self, pos: Position, piece: Piece) {
        self.pieces[pos.0][pos.1] = Some(piece);
    }

    pub fn destroy_piece_at(&mut self, pos: Position) {
        self.pieces[pos.0][pos.1] = None;
    }

    pub fn piece_at(&self, pos: Position) -> Option<Piece> {
        self.pieces[pos.0][pos.1]
    }

    /// Calculates all moves for the piece at the specified
    /// position, panicking if there is no piece at that position.
    pub fn possible_moves(&self, pos: Position) -> Vec<Move> {
        let piece = self.piece_at(pos).expect(&format!("No piece at {:?}", pos));

        let mut dests = vec![];

        match piece.ty() {
            PieceType::Pawn => {
                // Check forward
                if self.piece_at(pos.up()) == None {
                    dests.push(pos.up());
                }

                // Forward + diagonal
                if let Some(enemy) = self.piece_at(pos.up().left()) {
                    if enemy.color() == piece.color.opposite() {
                        dests.push(pos.up().left());
                    }
                }

                if let Some(enemy) = self.piece_at(pos.up().right()) {
                    if enemy.color() == piece.color.opposite() {
                        dests.push(pos.up().right());
                    }
                }
            },
            PieceType::Bishop => {

            },
            _ => unimplemented!()
        }

        dests.iter().map(|val| Move::new(pos, *val)).collect()
    }
}

#[derive(Clone, Copy, Debug, new)]
pub struct Move {
    from: Position,
    to: Position,
}

#[derive(Clone, Copy, Debug)]
pub struct Position(usize, usize);

impl Position {
    pub fn up(&self) -> Self {
        Position(self.0, self.1 + 1)
    }

    pub fn down(&self) -> Self {
        Position(self.0, self.1 - 1)
    }

    pub fn right(&self) -> Self {
        Position(self.0 + 1, self.1)
    }

    pub fn left(&self) -> Self {
        Position(self.0 - 1, self.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_board_setup() {
        let board = Chessboard::new();

        assert_eq!(board.piece_at(Position(0, 0)), Some(Piece::new(PieceType::Rook, Color::White)));
        assert_eq!(board.piece_at(Position(5, 5)), None);
        assert_eq!(board.piece_at(Position(4, 1)), Some(Piece::new(PieceType::Pawn, Color::White)));
    }

    #[test]
    fn test_set_piece_at() {
        let mut board = Chessboard::new();

        board.set_piece_at(Position(5, 5), Piece::new(PieceType::Bishop, Color::Black));
        assert_eq!(board.piece_at(Position(5, 5)), Some(Piece::new(PieceType::Bishop, Color::Black)));
    }

    #[test]
    fn test_destroy_piece() {
        let mut board = Chessboard::new();
        assert_eq!(board.piece_at(Position(0, 0)), Some(Piece::new(PieceType::Rook, Color::White)));
        board.destroy_piece_at(Position(0, 0));
        assert_eq!(board.piece_at(Position(0, 0)), None);
    }

    #[bench]
    fn bench_board_setup(b: &mut Bencher) {
        b.iter(|| Chessboard::new());
    }
}
