use ansi_term::Color::{Green, Red};
use std::fmt::Display;

use crate::{chess_logic::Player, parameters::PARAMETERS};

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
    pub fn value(&self) -> f32 {
        match self {
            Piece::Pawn => PARAMETERS.pawn_value,
            Piece::Bishop => PARAMETERS.bishop_value,
            Piece::Knight => PARAMETERS.knight_value,
            Piece::Rook => PARAMETERS.rook_value,
            Piece::Queen => PARAMETERS.queen_value,
            Piece::King => PARAMETERS.knight_value,
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
