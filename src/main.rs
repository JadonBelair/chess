use piece::{Piece, PieceType, Team};
use std::fmt::Debug;
mod piece;

struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Pos {
    pub fn new(x: usize, y: usize) -> Self {
        Self {x, y}
    }
}

impl Debug for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let mut board = build_board();
    board[3][3] = Some(Piece::new(PieceType::Rook, Team::Black));
    board[3][1] = Some(Piece::new(PieceType::Rook, Team::Black));

    for row in (0..8).rev() {
        for col in 0..8 {
            if let Some(p) = board[row][col] {
                print!("{} ", p);
            } else {
                print!("-- ");
            }
        }
        println!();
    }

    println!("{:?}", get_piece_moves(&board, 3, 3));
}

fn build_board() -> [[Option<Piece>; 8]; 8] {
    let mut board: [[Option<Piece>; 8]; 8] = [[None; 8]; 8];

    let pieces = vec![
        PieceType::Rook,
        PieceType::Knight,
        PieceType::Bishop,
        PieceType::Queen,
        PieceType::King,
        PieceType::Bishop,
        PieceType::Knight,
        PieceType::Rook,
    ];

    for i in 0..8 {
        board[1][i] = Some(Piece::new(PieceType::Pawn, Team::White));
        board[6][i] = Some(Piece::new(PieceType::Pawn, Team::Black));

        board[0][i] = Some(Piece::new(pieces[i], Team::White));
        board[7][i] = Some(Piece::new(pieces[i], Team::Black));
    }
    
    return board;
}

fn get_piece_moves(board: &[[Option<Piece>; 8]; 8], col: usize, row: usize) -> Vec<Pos> {
    let mut possible_spaces = Vec::new();

    if let Some(piece) = board[row][col] {
        match piece.piece {
            PieceType::Pawn => {
                match piece.team {
                    Team::White => {
                        if row < 7 && board[row+1][col].is_none() {
                            possible_spaces.push(Pos::new(col, row+1));

                            // checks if this is pieces first move
                            // and allows to move 2 spaces
                            if row == 1 && board[3][col].is_none() {
                                possible_spaces.push(Pos::new(col, 3));
                            }
                        }
                        
                        // allows capturing
                        if row < 7 && col > 0 && board[row+1][col-1].is_some() {
                            let other_team = board[row+1][col-1].expect(format!("{} {} should not have been none but it somehow is", col-1, row+1).as_str()).team;
                            if other_team != piece.team { possible_spaces.push(Pos::new(col-1, row+1)); }
                        }
                        if row < 7 && col < 7 && board[row+1][col+1].is_some() {
                            let other_team = board[row+1][col+1].expect(format!("{} {} should not have been none but it somehow is", col+1, row+1).as_str()).team;
                            if other_team != piece.team { possible_spaces.push(Pos::new(col+1, row+1)); }
                        }
                    },
                    Team::Black => {
                        if row > 0 && board[row-1][col].is_none() {
                            possible_spaces.push(Pos::new(col, row-1));

                            // checks if this is pieces first move
                            // and allows to move 2 spaces
                            if row == 6 && board[4][col].is_none() {
                                possible_spaces.push(Pos::new(col, 4));
                            }
                        }

                        // allows capturing
                        if row > 0 && col > 0 && board[row-1][col-1].is_some() {
                            let other_team = board[row-1][col-1].expect(format!("{} {} should not have been none but it somehow is", col-1, row-1).as_str()).team;
                            if other_team != piece.team { possible_spaces.push(Pos::new(col-1, row-1)); }
                        }
                        if row > 0 && col < 7 && board[row-1][col+1].is_some() {
                            let other_team = board[row-1][col+1].expect(format!("{} {} should not have been none but it somehow is", col+1, row-1).as_str()).team;
                            if other_team != piece.team { possible_spaces.push(Pos::new(col+1, row-1)); }
                        }
                    }
                };
            },
            PieceType::Rook => {
                for i in (col+1)..8 {
                    if let Some(other) = board[row][i] {
                        if other.team != piece.team {
                            possible_spaces.push(Pos::new(i, row));
                        }
                        break;
                    }
                    possible_spaces.push(Pos::new(i, row));
                }

                for i in (0..col).rev() {
                    if let Some(other) = board[row][i] {
                        if other.team != piece.team {
                            possible_spaces.push(Pos::new(i, row));
                        }
                        break;
                    }
                    possible_spaces.push(Pos::new(i, row));
                }

                for i in (row+1)..8 {
                    if let Some(other) = board[i][col] {
                        if other.team != piece.team {
                            possible_spaces.push(Pos::new(col, i));
                        }
                        break;
                    }
                    possible_spaces.push(Pos::new(col, i));
                }
                

                for i in (0..row).rev() {
                    if let Some(other) = board[i][col] {
                        if other.team != piece.team {
                            possible_spaces.push(Pos::new(col, i));
                        }
                        break;
                    }
                    possible_spaces.push(Pos::new(col, i));
                }
            },
            _ => ()
        };
    }

    return possible_spaces;
}