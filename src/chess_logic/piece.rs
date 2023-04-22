use ansi_term::Color::{Green, Red};
use std::fmt::Display;

use super::Player;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Piece {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Piece::Pawn => "P",
                Piece::Bishop => "B",
                Piece::Knight => "N",
                Piece::Rook => "R",
                Piece::Queen => "Q",
                Piece::King => "K",
            }
        )
    }
}

impl Piece {
    pub const fn value(&self) -> f32 {
        match self {
            Piece::Pawn => 1.0,
            Piece::Bishop => 3.0,
            Piece::Knight => 3.0,
            Piece::Rook => 5.0,
            Piece::Queen => 9.0,
            Piece::King => f32::MAX,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct OwnedPiece {
    player: Player,
    piece: Piece,
}

impl OwnedPiece {
    pub fn new(player: Player, piece: Piece) -> Self {
        Self { player, piece }
    }
}

impl Display for OwnedPiece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.player {
            Player::White => write!(f, "{}", Green.paint(self.piece.to_string())),
            Player::Black => write!(f, "{}", Red.paint(self.piece.to_string())),
        }
    }
}
