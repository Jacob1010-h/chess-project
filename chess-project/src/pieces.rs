use strum_macros::EnumIter;
extern crate serde_json;

#[derive(Debug, Clone, Copy, serde::Serialize, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, serde::Serialize, EnumIter)]
pub enum PieceType {
    Pawn = 1,
    Knight = 2,
    Bishop = 3,
    Rook = 4,
    Queen = 5,
    King = 6,
}

#[derive(Debug, Clone, Copy, serde::Serialize)]
pub struct Piece {
    pub color: Color,
    pub piece_type: PieceType,
}

impl Piece {
    pub(crate) fn get_value(&self) -> usize {
        match self.piece_type {
            PieceType::Pawn => 1,
            PieceType::Knight => 3,
            PieceType::Bishop => 3,
            PieceType::Rook => 5,
            PieceType::Queen => 9,
            PieceType::King => usize::MAX,
        }
    }

    pub(crate) fn get_piece_type_as_string(&self) -> String {
        match self.piece_type {
            PieceType::Pawn => "Pawn".to_string(),
            PieceType::Knight => "Knight".to_string(),
            PieceType::Bishop => "Bishop".to_string(),
            PieceType::Rook => "Rook".to_string(),
            PieceType::Queen => "Queen".to_string(),
            PieceType::King => "King".to_string(),
        }
    }

    pub(crate) fn get_pawn_moves(&self, position: (usize, usize)) -> Vec<(usize, usize)> {
        let mut moves = vec![];
        let direction = if self.color == Color::White { -1 } else { 1 };
        let new_row = position.0 as isize + direction;

        // Check if the new position is within bounds before casting back to usize
        if new_row >= 0 {
            moves.push((new_row as usize, position.1));
        }

        moves
    }

    pub(crate) fn get_knight_moves(&self, position: (usize, usize)) -> Vec<(usize, usize)> {
        let mut moves = vec![];
        let (row, col) = position;

        // All possible L-shaped moves
        let potential_moves = [
            (row as isize + 2, col as isize + 1),
            (row as isize + 2, col as isize - 1),
            (row as isize - 2, col as isize + 1),
            (row as isize - 2, col as isize - 1),
            (row as isize + 1, col as isize + 2),
            (row as isize + 1, col as isize - 2),
            (row as isize - 1, col as isize + 2),
            (row as isize - 1, col as isize - 2),
        ];

        for (r, c) in potential_moves {
            if (0..8).contains(&r) && (0..8).contains(&c) {
                moves.push((r as usize, c as usize));
            }
        }

        moves
    }

    pub(crate) fn get_bishop_moves(&self, position: (usize, usize)) -> Vec<(usize, usize)> {
        let mut moves = vec![];
        let (row, col) = position;

        let directions = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

        for (dr, dc) in directions {
            let mut r = row as isize + dr;
            let mut c = col as isize + dc;

            while (0..8).contains(&r) && (0..8).contains(&c) {
                moves.push((r as usize, c as usize));
                // Add blocking piece logic here
                r += dr;
                c += dc;
            }
        }

        moves
    }

    pub(crate) fn get_rook_moves(&self, position: (usize, usize)) -> Vec<(usize, usize)> {
        let mut moves = vec![];
        let (row, col) = position;

        let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];

        for (dr, dc) in directions {
            let mut r = row as isize + dr;
            let mut c = col as isize + dc;

            while (0..8).contains(&r) && (0..8).contains(&c) {
                moves.push((r as usize, c as usize));
                // Add blocking piece logic here
                r += dr;
                c += dc;
            }
        }

        moves
    }

    pub(crate) fn get_queen_moves(&self, position: (usize, usize)) -> Vec<(usize, usize)> {
        let mut moves = self.get_rook_moves(position);
        moves.extend(self.get_bishop_moves(position));
        moves
    }

    pub(crate) fn get_king_moves(&self, position: (usize, usize)) -> Vec<(usize, usize)> {
        let mut moves = vec![];
        let (row, col) = position;

        for dr in -1..=1 {
            for dc in -1..=1 {
                if dr == 0 && dc == 0 {
                    continue;
                }

                let r = row as isize + dr;
                let c = col as isize + dc;

                if (0..8).contains(&r) && (0..8).contains(&c) {
                    moves.push((r as usize, c as usize));
                }
            }
        }

        moves
    }

    pub fn get_legal_moves(&self, position: (usize, usize)) -> Vec<(usize, usize)> {
        match self.piece_type {
            PieceType::Pawn => self.get_pawn_moves(position),
            PieceType::Knight => self.get_knight_moves(position),
            PieceType::Bishop => self.get_bishop_moves(position),
            PieceType::Rook => self.get_rook_moves(position),
            PieceType::Queen => self.get_queen_moves(position),
            PieceType::King => self.get_king_moves(position),
        }
    }

    pub(crate) fn get_color(&self) -> Color {
        self.color
    }

    pub(crate) fn get_piece_type(&self) -> PieceType {
        self.piece_type
    }

    #[allow(dead_code)]
    pub fn new(color: Color, piece_type: PieceType) -> Self {
        Piece { color, piece_type }
    }

    #[allow(dead_code)]
    pub fn w_pawn() -> Self {
        Piece::new(Color::White, PieceType::Pawn)
    }
    #[allow(dead_code)]
    pub fn w_knight() -> Self {
        Piece::new(Color::White, PieceType::Knight)
    }
    #[allow(dead_code)]
    pub fn w_bishop() -> Self {
        Piece::new(Color::White, PieceType::Bishop)
    }
    #[allow(dead_code)]
    pub fn w_rook() -> Self {
        Piece::new(Color::White, PieceType::Rook)
    }
    #[allow(dead_code)]
    pub fn w_queen() -> Self {
        Piece::new(Color::White, PieceType::Queen)
    }
    #[allow(dead_code)]
    pub fn w_king() -> Self {
        Piece::new(Color::White, PieceType::King)
    }
    #[allow(dead_code)]
    pub fn b_pawn() -> Self {
        Piece::new(Color::Black, PieceType::Pawn)
    }
    #[allow(dead_code)]
    pub fn b_knight() -> Self {
        Piece::new(Color::Black, PieceType::Knight)
    }
    #[allow(dead_code)]
    pub fn b_bishop() -> Self {
        Piece::new(Color::Black, PieceType::Bishop)
    }
    #[allow(dead_code)]
    pub fn b_rook() -> Self {
        Piece::new(Color::Black, PieceType::Rook)
    }
    #[allow(dead_code)]
    pub fn b_queen() -> Self {
        Piece::new(Color::Black, PieceType::Queen)
    }
    #[allow(dead_code)]
    pub fn b_king() -> Self {
        Piece::new(Color::Black, PieceType::King)
    }
}
