extern crate serde_json;
use crate::pieces::{Color, Piece, PieceType};

pub type Move = (Piece, (usize, usize), (usize, usize));
pub type MoveHistory = Vec<Move>;

// --- Game struct and impl ---
#[derive(Debug)]
pub struct Game {
    board: Board,
    current_turn: Color,
    game_over: bool,
    move_history: MoveHistory,
}
impl Default for Game {
    fn default() -> Self {
        let mut board = Board::default();
        board.new();
        Self {
            board,
            current_turn: Color::White,
            game_over: false,
            move_history: Vec::new(),
        }
    }
}
impl Game {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn get_current_turn(&self) -> Color {
        self.current_turn
    }

    pub fn is_game_over(&self) -> bool {
        self.game_over
    }

    pub fn get_move_history(&self) -> &MoveHistory {
        &self.move_history
    }

    pub fn make_move(&mut self, piece: Piece, from: (usize, usize), to: (usize, usize)) {
        // Update the board and move history
        self.board.squares[to.0][to.1] = Some(piece);
        self.board.squares[from.0][from.1] = None;
        self.move_history.push((piece, from, to));

        // Switch turns
        self.current_turn = self.current_turn.opposite();
    }
}

// --- Board struct and impl ---
#[derive(Default, Debug, Clone, serde::Serialize)]
pub struct Board {
    squares: [[Option<Piece>; 8]; 8],
}
impl Board {
    pub fn new(&mut self) -> &mut Self {
        // clear the board
        for row in self.squares.iter_mut() {
            for square in row.iter_mut() {
                *square = None;
            }
        }

        // Set up the initial position of the pieces
        for (row, squares) in self.squares.iter_mut().enumerate() {
            for (col, square) in squares.iter_mut().enumerate() {
                if row == 6 {
                    *square = Some(Piece {
                        color: Color::White,
                        piece_type: PieceType::Pawn,
                    });
                } else if row == 1 {
                    *square = Some(Piece {
                        color: Color::Black,
                        piece_type: PieceType::Pawn,
                    });
                } else if row == 0 || row == 7 {
                    let color = if row == 0 { Color::Black } else { Color::White };
                    match col {
                        0 | 7 => {
                            *square = Some(Piece {
                                color,
                                piece_type: PieceType::Rook,
                            })
                        }
                        1 | 6 => {
                            *square = Some(Piece {
                                color,
                                piece_type: PieceType::Knight,
                            })
                        }
                        2 | 5 => {
                            *square = Some(Piece {
                                color,
                                piece_type: PieceType::Bishop,
                            })
                        }
                        3 => {
                            *square = Some(Piece {
                                color,
                                piece_type: PieceType::Queen,
                            })
                        }
                        4 => {
                            *square = Some(Piece {
                                color,
                                piece_type: PieceType::King,
                            })
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }
        self
    }

    pub fn get_piece_at(&self, position: (usize, usize)) -> Option<&Piece> {
        if position.0 < 8 && position.1 < 8 {
            self.squares[position.0][position.1].as_ref()
        } else {
            None
        }
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.squares.len(), self.squares[0].len())
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn pretty_print(&self) {
        for row in self.squares.iter() {
            for square in row.iter() {
                match square {
                    Some(piece) => {
                        // ♔♕♖♗♘♙  ♚♛♜♝♞♟
                        let symbol = match piece.piece_type {
                            PieceType::King => {
                                if piece.color == Color::White {
                                    '♔'
                                } else {
                                    '♚'
                                }
                            }
                            PieceType::Queen => {
                                if piece.color == Color::White {
                                    '♕'
                                } else {
                                    '♛'
                                }
                            }
                            PieceType::Rook => {
                                if piece.color == Color::White {
                                    '♖'
                                } else {
                                    '♜'
                                }
                            }
                            PieceType::Bishop => {
                                if piece.color == Color::White {
                                    '♗'
                                } else {
                                    '♝'
                                }
                            }
                            PieceType::Knight => {
                                if piece.color == Color::White {
                                    '♘'
                                } else {
                                    '♞'
                                }
                            }
                            PieceType::Pawn => {
                                if piece.color == Color::White {
                                    '♙'
                                } else {
                                    '♟'
                                }
                            }
                        };
                        print!("{} ", symbol);
                    }
                    None => print!(". "),
                }
            }
            println!();
        }
    }
}
