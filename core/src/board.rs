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
        self.pieces[pos.0 as usize][pos.1 as usize] = Some(piece);
    }

    pub fn destroy_piece_at(&mut self, pos: Position) {
        self.pieces[pos.0 as usize][pos.1 as usize] = None;
    }

    pub fn piece_at(&self, pos: Position) -> Option<Piece> {
        self.pieces[pos.0 as usize][pos.1 as usize]
    }

    /// Calculates all moves for the piece at the specified
    /// position, panicking if there is no piece at that position.
    pub fn possible_moves(&self, pos: Position) -> Vec<Move> {
        let piece = self.piece_at(pos).expect(&format!("No piece at {:?}", pos));

        let mut dests = vec![];

        match piece.ty() {
            PieceType::Pawn => {
                // Check forward
                match piece.color() {
                    Color::White => {
                        if self.piece_at(pos.up()) == None {
                            dests.push(pos.up());
                        }
                    }
                    Color::Black => {
                        if self.piece_at(pos.down()) == None {
                            dests.push(pos.down());
                        }
                    }
                }

                // If in correct position, check two up / two down
                match piece.color() {
                    Color::White => {
                        if pos.1 == 1 && self.piece_at(pos.up().up()) == None {
                            dests.push(pos.up().up());
                        }
                    }
                    Color::Black => {
                        if pos.1 == 6 && self.piece_at(pos.down().down()) == None {
                            dests.push(pos.down().down());
                        }
                    }
                }

                // Forward + diagonal
                match piece.color() {
                    Color::White => {
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
                    }
                    Color::Black => {
                        if let Some(enemy) = self.piece_at(pos.down().left()) {
                            if enemy.color() == piece.color.opposite() {
                                dests.push(pos.down().left());
                            }
                        }

                        if let Some(enemy) = self.piece_at(pos.down().right()) {
                            if enemy.color() == piece.color.opposite() {
                                dests.push(pos.down().right());
                            }
                        }
                    }
                }
            }
            PieceType::Bishop => {
                compute_diagonals(pos, piece.color(), self, &mut dests);
            }
            PieceType::Rook => {
                compute_straight(pos, piece.color(), self, &mut dests);
            }
            PieceType::Queen => {
                compute_straight(pos, piece.color(), self, &mut dests);
                compute_diagonals(pos, piece.color(), self, &mut dests);
            }
            PieceType::King => {
                let possible_dests = [
                    pos.up(),
                    pos.down(),
                    pos.left(),
                    pos.right(),
                    pos.up().right(),
                    pos.up().left(),
                    pos.down().right(),
                    pos.down().left(),
                ];

                possible_dests.iter().for_each(|dest| {
                    if allowed_move(*dest, piece.color(), self) {
                        dests.push(*dest);
                    }
                });
            }
            PieceType::Knight => {
                let possible_dests = [
                    pos.up().up().right(),
                    pos.up().up().left(),
                    pos.down().down().right(),
                    pos.down().down().left(),
                    pos.right().right().up(),
                    pos.right().right().down(),
                    pos.left().left().up(),
                    pos.left().left().down(),
                ];

                possible_dests.iter().for_each(|dest| {
                    if allowed_move(*dest, piece.color(), self) {
                        dests.push(*dest);
                    }
                });
            }
        }

        dests.iter().map(|val| Move::new(pos, *val)).collect()
    }
}

fn allowed_move(to: Position, piece_color: Color, board: &Chessboard) -> bool {
    if !to.is_valid() {
        return false;
    }

    if let Some(piece) = board.piece_at(to) {
        piece.color() == piece_color.opposite()
    } else {
        true
    }
}

fn compute_diagonals(pos: Position, color: Color, board: &Chessboard, dests: &mut Vec<Position>) {
    // Move diagonally in each of the four
    // directions until an obstacle is found.
    // If the obstacle is an enemy, the obstacle
    // is added to the possible destinations, and it is the last entry there;
    // otherwise, it is not counted as a possible destination.

    offsets(pos, color, board, dests, (1, 1));
    offsets(pos, color, board, dests, (1, -1));
    offsets(pos, color, board, dests, (-1, 1));
    offsets(pos, color, board, dests, (-1, -1));
}

fn compute_straight(pos: Position, color: Color, board: &Chessboard, dests: &mut Vec<Position>) {
    // Same thing as diagonals, but straight instead of diagonal
    offsets(pos, color, board, dests, (0, 1));
    offsets(pos, color, board, dests, (1, 0));
    offsets(pos, color, board, dests, (0, -1));
    offsets(pos, color, board, dests, (-1, 0));
}

fn offsets(
    pos: Position,
    color: Color,
    board: &Chessboard,
    dests: &mut Vec<Position>,
    offset: (isize, isize),
) {
    let mut p = pos;
    while p.offset(offset).is_valid() {
        p = p.offset(offset);

        if let Some(piece) = board.piece_at(p) {
            if piece.color() == color.opposite() {
                dests.push(p);
                break;
            } else {
                break;
            }
        }

        dests.push(p);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, new)]
pub struct Move {
    from: Position,
    to: Position,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Position(isize, isize);

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

    pub fn is_valid(&self) -> bool {
        self.0 < 8 && self.0 >= 0 && self.1 < 8 && self.1 >= 0
    }

    pub fn offset(&self, offset: (isize, isize)) -> Self {
        Position(self.0 + offset.0, self.1 + offset.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_valid_moves() {
        let board = Chessboard::new();

        {
            let pos = Position(4, 1);
            let pawn_moves = board.possible_moves(pos);
            assert!(pawn_moves.contains(&Move::new(pos, Position(4, 2))));
            assert!(pawn_moves.contains(&Move::new(pos, Position(4, 3))));
            assert_eq!(pawn_moves.len(), 2);
        }
        {
            let pos = Position(0, 0);
            let rook_moves = board.possible_moves(pos);
            assert_eq!(rook_moves.len(), 0);
        }
        {
            let pos = Position(1, 0);
            let knight_moves = board.possible_moves(pos);
            assert!(knight_moves.contains(&Move::new(pos, Position(0, 2))));
            assert!(knight_moves.contains(&Move::new(pos, Position(2, 2))));
            assert_eq!(knight_moves.len(), 2);
        }
    }

    #[test]
    fn test_board_setup() {
        let board = Chessboard::new();

        assert_eq!(
            board.piece_at(Position(0, 0)),
            Some(Piece::new(PieceType::Rook, Color::White))
        );
        assert_eq!(board.piece_at(Position(5, 5)), None);
        assert_eq!(
            board.piece_at(Position(4, 1)),
            Some(Piece::new(PieceType::Pawn, Color::White))
        );
    }

    #[test]
    fn test_set_piece_at() {
        let mut board = Chessboard::new();

        board.set_piece_at(Position(5, 5), Piece::new(PieceType::Bishop, Color::Black));
        assert_eq!(
            board.piece_at(Position(5, 5)),
            Some(Piece::new(PieceType::Bishop, Color::Black))
        );
    }

    #[test]
    fn test_destroy_piece() {
        let mut board = Chessboard::new();
        assert_eq!(
            board.piece_at(Position(0, 0)),
            Some(Piece::new(PieceType::Rook, Color::White))
        );
        board.destroy_piece_at(Position(0, 0));
        assert_eq!(board.piece_at(Position(0, 0)), None);
    }

    #[test]
    fn test_compute_diagonals() {
        let mut dests = vec![];
        let color = Color::Black;
        let mut board = Chessboard::new();
        board.set_piece_at(Position(3, 3), Piece::new(PieceType::Bishop, Color::Black));

        compute_diagonals(Position(3, 3), color, &board, &mut dests);

        assert!(dests.contains(&Position(4, 4)));
        assert!(dests.contains(&Position(5, 5)));
        assert!(!dests.contains(&Position(6, 6)));

        assert!(dests.contains(&Position(2, 2)));
        assert!(dests.contains(&Position(1, 1)));
        assert!(!dests.contains(&Position(0, 0)));

        assert!(dests.contains(&Position(2, 4)));
        assert!(dests.contains(&Position(1, 5)));
        assert!(!dests.contains(&Position(0, 6)));

        assert!(!dests.contains(&Position(7, 7)));
    }

    #[test]
    fn test_compute_straight() {
        let mut dests = vec![];
        let color = Color::Black;
        let mut board = Chessboard::new();
        board.set_piece_at(Position(3, 3), Piece::new(PieceType::Bishop, Color::Black));

        compute_straight(Position(3, 3), color, &board, &mut dests);

        assert!(dests.contains(&Position(3, 4)));
        assert!(dests.contains(&Position(3, 5)));
        assert!(!dests.contains(&Position(3, 8))); // Invalid!

        assert!(dests.contains(&Position(3, 2)));
        assert!(dests.contains(&Position(3, 1)));
        assert!(!dests.contains(&Position(3, 0)));
    }

    #[test]
    fn test_allowed_move() {
        let board = Chessboard::new();
        let color = Color::White;

        assert!(allowed_move(Position(0, 6), color, &board));
        assert!(allowed_move(Position(4, 4), color, &board));

        assert!(!allowed_move(Position(0, 1), color, &board));
        assert!(!allowed_move(Position(8, 0), color, &board));
    }

    #[bench]
    fn bench_board_setup(b: &mut Bencher) {
        b.iter(|| Chessboard::new());
    }

    #[bench]
    fn bench_compute_diagonals(b: &mut Bencher) {
        let board = Chessboard::new();
        b.iter(|| {
            let mut dests = vec![];
            compute_diagonals(Position(3, 3), Color::Black, &board, &mut dests);
        });
    }

    #[bench]
    fn bench_compute_straights(b: &mut Bencher) {
        let board = Chessboard::new();
        b.iter(|| {
            let mut dests = vec![];
            compute_straight(Position(3, 3), Color::Black, &board, &mut dests);
        })
    }
}
