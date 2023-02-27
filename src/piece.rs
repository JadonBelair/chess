use std::fmt::Display;

#[derive(Copy, Clone)]
pub enum PieceType {
	King,
	Queen,
	Knight,
	Bishop,
	Rook,
	Pawn,
}

#[derive(PartialEq, Copy, Clone)]
pub enum Team {
	White,
	Black,
}

#[derive(Copy, Clone)]
pub struct Piece {
	pub piece: PieceType,
	pub team: Team
}

impl Piece {
	pub fn new(piece: PieceType, team: Team) -> Self {
		Self {
			piece,
			team
		}
	}
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let team = if self.team == Team::White {
			"W"
		} else {
			"B"
		};

        write!(f, "{}{}", team, match self.piece {
            PieceType::King => "K",
            PieceType::Queen => "Q",
            PieceType::Knight => "N",
            PieceType::Bishop => "B",
            PieceType::Rook => "R",
            PieceType::Pawn => "P",
        })
    }
}