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
