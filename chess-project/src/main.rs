extern crate serde_json;
pub mod pieces;
use crate::pieces::{Color, Piece, PieceType};
use strum::IntoEnumIterator;

#[derive(Debug)]
struct Game {
    board: Board,
    current_turn: Color,
    game_over: bool,
}

#[derive(Default, Debug, serde::Serialize)]
struct Board {
    squares: [[Option<Piece>; 8]; 8],
}

// ---- Function Definitions ----

fn pretty_print_board(board: &Board) {
    for row in board.squares.iter() {
        for square in row.iter() {
            match square {
                Some(piece) => {
                    let piece_char = match piece.piece_type {
                        PieceType::Pawn => 'P',
                        PieceType::Knight => 'N',
                        PieceType::Bishop => 'B',
                        PieceType::Rook => 'R',
                        PieceType::Queen => 'Q',
                        PieceType::King => 'K',
                    };
                    let color_char = match piece.color {
                        Color::White => piece_char.to_ascii_uppercase(),
                        Color::Black => piece_char.to_ascii_lowercase(),
                    };
                    print!("{} ", color_char);
                }
                None => print!(". "),
            }
        }
        println!();
    }
}

fn create_starting_board_state(board: &mut Board) {
    for (row, squares) in board.squares.iter_mut().enumerate() {
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
}

fn pretty_print_piece_values() {
    println!("Piece Values:");
    for piece_type in PieceType::iter() {
        let piece = Piece {
            color: Color::White,
            piece_type,
        };
        let value = piece.get_value();
        println!("{:?}: {}", piece_type, value);
    }
}

fn create_json_from_board(board: &Board) -> String {
    serde_json::to_string(board).unwrap()
}

fn main() {
    let mut game = Game {
        board: Board::default(),
        current_turn: Color::White,
        game_over: false,
    };

    create_starting_board_state(&mut game.board);
    pretty_print_board(&game.board);

    pretty_print_piece_values();

    // println!("JSON representation of the board:");
    // let json_board = create_json_from_board(&game.board);
    // println!("{}", json_board);

    println!("Welcome to the Chess Game!");
}
