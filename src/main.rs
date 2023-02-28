use piece::{Piece, PieceType, Team};
use std::fmt::Debug;
mod piece;

#[derive(PartialEq, Clone, Copy)]
struct Pos2D {
    pub x: usize,
    pub y: usize,
}

impl Pos2D {
    pub fn new(x: usize, y: usize) -> Self {
        Self {x, y}
    }
}

impl Debug for Pos2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let /* mut */ board = /* [[None; 8]; 8]; */ build_board();
    // board[5][5] = Some(Piece::new(PieceType::Queen, Team::Black));
    // board[5][4] = Some(Piece::new(PieceType::Rook, Team::White));

    let moves = get_piece_moves(&board, Pos2D::new(1, 0));

    for row in (0..8).rev() {
        for col in 0..8 {
            if moves.contains(&Pos2D::new(col, row)) {
                print!("XX ");
            } else if let Some(p) = board[row][col] {
                print!("{} ", p);
            } else {
                print!("-- ");
            }
        }
        println!();
    }
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

fn get_piece_moves(board: &[[Option<Piece>; 8]; 8], current: Pos2D) -> Vec<Pos2D> {
    let mut possible_spaces = Vec::new();

    let col = current.x;
    let row = current.y;

    if let Some(piece) = board[row][col] {
        match piece.piece {
            PieceType::Pawn => {
                possible_spaces = generate_pawn_moves(board, piece, current);
            },
            PieceType::Rook => {
                possible_spaces = generate_rook_moves(board, piece, current);
            },
            PieceType::Knight => {
                possible_spaces = generate_knight_moves(board, piece, current);
            },
            PieceType::Bishop => {
                possible_spaces = generate_bishop_moves(board, piece, current);
            },
            PieceType::King => {
                possible_spaces = generate_king_moves(board, piece, current);
            },
            PieceType::Queen => {
                possible_spaces = generate_queen_moves(board, piece, current);
            }
        }
    }

    return possible_spaces;
}

fn generate_queen_moves(board: &[[Option<Piece>; 8]; 8], piece: Piece, pos: Pos2D) -> Vec<Pos2D> {
    let mut possible_spaces = Vec::new();

    possible_spaces.append(&mut generate_king_moves(board, piece, pos));
    possible_spaces.append(&mut generate_rook_moves(board, piece, pos));
    possible_spaces.append(&mut generate_bishop_moves(board, piece, pos));

    return possible_spaces;
}

fn generate_king_moves(board: &[[Option<Piece>; 8]; 8], piece: Piece, pos: Pos2D) -> Vec<Pos2D> {
    let row = pos.y;
    let col = pos.x;
    let mut possible_spaces = Vec::new();

    if row < 7 {
        if can_piece_enter(board, &piece, Pos2D::new(col, row+1)) { possible_spaces.push(Pos2D::new(col, row+1)); }
        if col > 0 && can_piece_enter(board, &piece, Pos2D::new(col-1, row+1)) { possible_spaces.push(Pos2D::new(col-1, row+1)); }
        if col < 7 && can_piece_enter(board, &piece, Pos2D::new(col+1, row+1)) { possible_spaces.push(Pos2D::new(col+1, row+1)); }
    }

    if col > 0 && can_piece_enter(board, &piece, Pos2D::new(col-1, row)) { possible_spaces.push(Pos2D::new(col-1, row)); }
    if col < 7 && can_piece_enter(board, &piece, Pos2D::new(col+1, row)) { possible_spaces.push(Pos2D::new(col+1, row)); }

    if row > 0 {
        if can_piece_enter(board, &piece, Pos2D::new(col, row-1)) { possible_spaces.push(Pos2D::new(col, row-1)); }
        if col > 0 && can_piece_enter(board, &piece, Pos2D::new(col-1, row-1)) { possible_spaces.push(Pos2D::new(col-1, row-1)); }
        if col < 7 && can_piece_enter(board, &piece, Pos2D::new(col+1, row-1)) { possible_spaces.push(Pos2D::new(col+1, row-1)); }
    }

    return possible_spaces;
}

fn generate_knight_moves(board: &[[Option<Piece>; 8]; 8], piece: Piece, pos: Pos2D) -> Vec<Pos2D> {
    let row = pos.y;
    let col = pos.x;
    let mut possible_spaces = Vec::new();

    if col > 0 && row < 6 && can_piece_enter(board, &piece, Pos2D::new(col-1, row+2)) {
        possible_spaces.push(Pos2D::new(col-1, row+2));
    }
    if col < 7 && row < 6 && can_piece_enter(board, &piece, Pos2D::new(col+1, row+2)) {
        possible_spaces.push(Pos2D::new(col+1, row+2));
    }
    if col > 1 && row < 7 && can_piece_enter(board, &piece, Pos2D::new(col-2, row+1)) {
        possible_spaces.push(Pos2D::new(col-2, row+1));
    }
    if col < 6 && row < 7 && can_piece_enter(board, &piece, Pos2D::new(col+2, row+1)) {
        possible_spaces.push(Pos2D::new(col+2, row+1));
    }
    if col > 1 && row > 0 && can_piece_enter(board, &piece, Pos2D::new(col-2, row-1)) {
        possible_spaces.push(Pos2D::new(col-2, row-1));
    }
    if col < 6 && row > 0 && can_piece_enter(board, &piece, Pos2D::new(col+2, row-1)) {
        possible_spaces.push(Pos2D::new(col+2, row-1));
    }
    if col > 0 && row > 1 && can_piece_enter(board, &piece, Pos2D::new(col-1, row-2)) {
        possible_spaces.push(Pos2D::new(col-1, row-2));
    }
    if col < 7 && row > 1 && can_piece_enter(board, &piece, Pos2D::new(col+1, row-2)){
        possible_spaces.push(Pos2D::new(col+1, row-2));
    }

    return possible_spaces;
}

fn generate_rook_moves(board: &[[Option<Piece>; 8]; 8], piece: Piece, pos: Pos2D) -> Vec<Pos2D> {
    let row = pos.y;
    let col = pos.x;
    let mut possible_spaces = Vec::new();

    // checks how far up the piece can move
    for i in (col+1)..8 {
        if can_piece_enter(board, &piece, Pos2D::new(i, row)) {
            possible_spaces.push(Pos2D::new(i, row));
            if board[row][i].is_some() { break; }
        } else {
            break;
        }
    }

    // checks how far down the piece can move
    for i in (0..col).rev() {
        if can_piece_enter(board, &piece, Pos2D::new(i, row)) {
            possible_spaces.push(Pos2D::new(i, row));
            if board[row][i].is_some() { break; }
        } else {
            break;
        }
    }

    // checks how far to the right the piece can move
    for i in (row+1)..8 {
        if can_piece_enter(board, &piece, Pos2D::new(col, i)) {
            possible_spaces.push(Pos2D::new(col, i));
            if board[i][col].is_some() { break; }
        } else {
            break;
        }
    }
    
    // checks how far to the left the piece can move
    for i in (0..row).rev() {
        if can_piece_enter(board, &piece, Pos2D::new(col, i)) {
            possible_spaces.push(Pos2D::new(col, i));
            if board[i][col].is_some() { break; }
        } else {
            break;
        }
    }

    return possible_spaces;
}

fn generate_bishop_moves(board: &[[Option<Piece>; 8]; 8], piece: Piece, pos: Pos2D) -> Vec<Pos2D> {
    let row = pos.y;
    let col = pos.x;
    let mut possible_spaces = Vec::new();

    for i in 1..(8-col) {
        if (row+i > 7) || (col+i > 7) {
            break;
        }

        if can_piece_enter(board, &piece, Pos2D::new(col+i, row+i)) {
            possible_spaces.push(Pos2D::new(col+i, row+i));

            if board[row+i][col+i].is_some() {
                break;
            }
        } else {
            break;
        }
    }

    for i in 1..(8-col) {
        if ((row as isize - i as isize) < 0) || (col+i > 7) {
            break;
        }

        if can_piece_enter(board, &piece, Pos2D::new(col+i, row-i)) {
            possible_spaces.push(Pos2D::new(col+i, row-i));

            if board[row-i][col+i].is_some() {
                break;
            }
        } else {
            break;
        }
    }

    for i in 1..=(col) {
        if (row+i > 7) || ((col as isize - i as isize) < 0) {
            break;
        }

        if can_piece_enter(board, &piece, Pos2D::new(col-i, row+i)) {
            possible_spaces.push(Pos2D::new(col-i, row+i));

            if board[row+i][col-i].is_some() {
                break;
            }
        } else {
            break;
        }
    }
    
    for i in 1..=(col) {
        if ((row as isize - i as isize) < 0) || ((col as isize - i as isize) < 0) {
            break;
        }

        if can_piece_enter(board, &piece, Pos2D::new(col-i, row-i)) {
            possible_spaces.push(Pos2D::new(col-i, row-i));

            if board[row-i][col-i].is_some() {
                break;
            }
        } else {
            break;
        }
    }

    return possible_spaces;
}

fn generate_pawn_moves(board: &[[Option<Piece>; 8]; 8], piece: Piece, pos: Pos2D) -> Vec<Pos2D> {
    let row = pos.y;
    let col = pos.x;
    let mut possible_spaces = Vec::new();

    match piece.team {
        Team::White => {
            if row < 7 && board[row+1][col].is_none() {
                possible_spaces.push(Pos2D::new(col, row+1));

                // checks if this is pieces first move
                // and allows to move 2 spaces
                if row == 1 && board[3][col].is_none() {
                    possible_spaces.push(Pos2D::new(col, 3));
                }
            }
            
            // allows capturing
            if row < 7 && col > 0 && board[row+1][col-1].is_some() {
                let other_team = board[row+1][col-1].expect(format!("{} {} should not have been none but it somehow is", col-1, row+1).as_str()).team;
                if other_team != piece.team { possible_spaces.push(Pos2D::new(col-1, row+1)); }
            }
            if row < 7 && col < 7 && board[row+1][col+1].is_some() {
                let other_team = board[row+1][col+1].expect(format!("{} {} should not have been none but it somehow is", col+1, row+1).as_str()).team;
                if other_team != piece.team { possible_spaces.push(Pos2D::new(col+1, row+1)); }
            }
        },
        Team::Black => {
            if row > 0 && board[row-1][col].is_none() {
                possible_spaces.push(Pos2D::new(col, row-1));

                // checks if this is pieces first move
                // and allows to move 2 spaces
                if row == 6 && board[4][col].is_none() {
                    possible_spaces.push(Pos2D::new(col, 4));
                }
            }

            // allows capturing
            if row > 0 && col > 0 && board[row-1][col-1].is_some() {
                let other_team = board[row-1][col-1].expect(format!("{} {} should not have been none but it somehow is", col-1, row-1).as_str()).team;
                if other_team != piece.team { possible_spaces.push(Pos2D::new(col-1, row-1)); }
            }
            if row > 0 && col < 7 && board[row-1][col+1].is_some() {
                let other_team = board[row-1][col+1].expect(format!("{} {} should not have been none but it somehow is", col+1, row-1).as_str()).team;
                if other_team != piece.team { possible_spaces.push(Pos2D::new(col+1, row-1)); }
            }
        }
    }

    return possible_spaces;
}

fn can_piece_enter(board: &[[Option<Piece>; 8]; 8], piece: &Piece, move_to: Pos2D) -> bool {
    if let Some(other) = board[move_to.y][move_to.x] {
        if other.team != piece.team {
            return true;
        } else {
            return false;
        }
    } else {
        return true;
    }
}