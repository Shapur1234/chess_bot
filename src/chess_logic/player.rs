use ansi_term::Color::{Green, Red};
use std::{fmt::Display, ops::Neg};

#[derive(Clone, Copy, Debug)]
pub enum Player {
    White,
    Black,
}

impl Neg for Player {
    type Output = Player;

    fn neg(self) -> Self::Output {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::White => write!(f, "{}", Green.paint("White".to_string())),
            Self::Black => write!(f, "{}", Red.paint("Black".to_string())),
        }
    }
}
