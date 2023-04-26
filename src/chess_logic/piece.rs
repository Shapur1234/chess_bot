use ansi_term::Color::{Green, Red};
use cgmath::Vector2;
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
    pub fn possible_move_dirs(&self) -> Vec<Vec<Vector2<i8>>> {
        match self.piece {
            Piece::Pawn => match self.player {
                Player::White => vec![
                    vec![Vector2::new(0, 1)],
                    vec![Vector2::new(-1, 1)],
                    vec![Vector2::new(1, 1)],
                ],
                Player::Black => vec![
                    vec![Vector2::new(0, -1)],
                    vec![Vector2::new(-1, -1)],
                    vec![Vector2::new(1, -1)],
                ],
            },
            Piece::Bishop => vec![
                vec![
                    Vector2::new(1, 1),
                    Vector2::new(2, 2),
                    Vector2::new(3, 3),
                    Vector2::new(4, 4),
                    Vector2::new(5, 5),
                    Vector2::new(6, 6),
                    Vector2::new(7, 7),
                ],
                vec![
                    Vector2::new(-1, 1),
                    Vector2::new(-2, 2),
                    Vector2::new(-3, 3),
                    Vector2::new(-4, 4),
                    Vector2::new(-5, 5),
                    Vector2::new(-6, 6),
                    Vector2::new(-7, 7),
                ],
                vec![
                    Vector2::new(1, -1),
                    Vector2::new(2, -2),
                    Vector2::new(3, -3),
                    Vector2::new(4, -4),
                    Vector2::new(5, -5),
                    Vector2::new(6, -6),
                    Vector2::new(7, -7),
                ],
                vec![
                    Vector2::new(-1, -1),
                    Vector2::new(-2, -2),
                    Vector2::new(-3, -3),
                    Vector2::new(-4, -4),
                    Vector2::new(-5, -5),
                    Vector2::new(-6, -6),
                    Vector2::new(-7, -7),
                ],
            ],
            Piece::Rook => vec![
                vec![
                    Vector2::new(1, 0),
                    Vector2::new(2, 0),
                    Vector2::new(3, 0),
                    Vector2::new(4, 0),
                    Vector2::new(5, 0),
                    Vector2::new(6, 0),
                    Vector2::new(7, 0),
                ],
                vec![
                    Vector2::new(-1, 0),
                    Vector2::new(-2, 0),
                    Vector2::new(-3, 0),
                    Vector2::new(-4, 0),
                    Vector2::new(-5, 0),
                    Vector2::new(-6, 0),
                    Vector2::new(-7, 0),
                ],
                vec![
                    Vector2::new(0, 1),
                    Vector2::new(0, 2),
                    Vector2::new(0, 3),
                    Vector2::new(0, 4),
                    Vector2::new(0, 5),
                    Vector2::new(0, 6),
                    Vector2::new(0, 7),
                ],
                vec![
                    Vector2::new(0, -1),
                    Vector2::new(0, -2),
                    Vector2::new(0, -3),
                    Vector2::new(0, -4),
                    Vector2::new(0, -5),
                    Vector2::new(0, -6),
                    Vector2::new(0, -7),
                ],
            ],
            Piece::Knight => vec![
                vec![Vector2::new(2, 1)],
                vec![Vector2::new(-2, 1)],
                vec![Vector2::new(2, -1)],
                vec![Vector2::new(-2, -1)],
                vec![Vector2::new(1, 2)],
                vec![Vector2::new(-1, 2)],
                vec![Vector2::new(1, -2)],
                vec![Vector2::new(-1, -2)],
            ],
            Piece::Queen => vec![
                vec![
                    Vector2::new(1, 1),
                    Vector2::new(2, 2),
                    Vector2::new(3, 3),
                    Vector2::new(4, 4),
                    Vector2::new(5, 5),
                    Vector2::new(6, 6),
                    Vector2::new(7, 7),
                ],
                vec![
                    Vector2::new(-1, 1),
                    Vector2::new(-2, 2),
                    Vector2::new(-3, 3),
                    Vector2::new(-4, 4),
                    Vector2::new(-5, 5),
                    Vector2::new(-6, 6),
                    Vector2::new(-7, 7),
                ],
                vec![
                    Vector2::new(1, -1),
                    Vector2::new(2, -2),
                    Vector2::new(3, -3),
                    Vector2::new(4, -4),
                    Vector2::new(5, -5),
                    Vector2::new(6, -6),
                    Vector2::new(7, -7),
                ],
                vec![
                    Vector2::new(-1, -1),
                    Vector2::new(-2, -2),
                    Vector2::new(-3, -3),
                    Vector2::new(-4, -4),
                    Vector2::new(-5, -5),
                    Vector2::new(-6, -6),
                    Vector2::new(-7, -7),
                ],
                vec![
                    Vector2::new(1, 0),
                    Vector2::new(2, 0),
                    Vector2::new(3, 0),
                    Vector2::new(4, 0),
                    Vector2::new(5, 0),
                    Vector2::new(6, 0),
                    Vector2::new(7, 0),
                ],
                vec![
                    Vector2::new(-1, 0),
                    Vector2::new(-2, 0),
                    Vector2::new(-3, 0),
                    Vector2::new(-4, 0),
                    Vector2::new(-5, 0),
                    Vector2::new(-6, 0),
                    Vector2::new(-7, 0),
                ],
                vec![
                    Vector2::new(0, 1),
                    Vector2::new(0, 2),
                    Vector2::new(0, 3),
                    Vector2::new(0, 4),
                    Vector2::new(0, 5),
                    Vector2::new(0, 6),
                    Vector2::new(0, 7),
                ],
                vec![
                    Vector2::new(0, -1),
                    Vector2::new(0, -2),
                    Vector2::new(0, -3),
                    Vector2::new(0, -4),
                    Vector2::new(0, -5),
                    Vector2::new(0, -6),
                    Vector2::new(0, -7),
                ],
            ],
            Piece::King => vec![
                vec![Vector2::new(0, 1)],
                vec![Vector2::new(1, 1)],
                vec![Vector2::new(1, 0)],
                vec![Vector2::new(1, -1)],
                vec![Vector2::new(0, -1)],
                vec![Vector2::new(-1, -1)],
                vec![Vector2::new(-1, 0)],
                vec![Vector2::new(1, 1)],
            ],
        }
    }
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
