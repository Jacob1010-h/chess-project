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

fn game_test() {
    let mut game = Game {
        board: Board::default(),
        current_turn: Color::White,
        game_over: false,
    };

    create_starting_board_state(&mut game.board);
    pretty_print_board(&game.board);

    pretty_print_piece_values();

    // Find the legal moves for a white pawn
    let white_pawn = Piece {
        color: Color::White,
        piece_type: PieceType::Pawn,
    };
    let pawn_position = (6, 0); // Starting position of the pawn
    let pawn_moves = white_pawn.get_legal_moves(pawn_position);
    println!(
        "Legal moves for white pawn at position {:?}: {:?}",
        pawn_position, pawn_moves
    );

    // // println!("JSON representation of the board:");
    // let json_board = create_json_from_board(&game.board);
    // println!("{}", json_board);

    println!("Welcome to the Chess Game!");
}
