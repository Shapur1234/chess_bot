use cgmath::{prelude::*, Vector2};
use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

const LINE_SIZE: u8 = 8;

#[derive(Clone, Copy, Debug)]
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
            "{:}",
            match self {
                Piece::Pawn => "p",
                Piece::Bishop => "b",
                Piece::Knight => "k",
                Piece::Rook => "r",
                Piece::Queen => "q",
                Piece::King => "K",
            }
        )
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Player {
    White,
    Black,
}

#[derive(Clone, Copy, Debug)]
pub struct OwnedPiece(Player, Piece);

impl Display for OwnedPiece {
    // TODO: Color

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:}", self.1)
    }
}

struct BoardIter {
    curr_x: u8,
    curr_y: u8,
}

impl BoardIter {
    pub fn new() -> Self {
        Self {
            curr_x: 255,
            curr_y: 1,
        }
    }
}

impl Iterator for BoardIter {
    type Item = Vector2<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: Beautify
        if self.curr_x == 255 {
            self.curr_x = 1;
            return Some(Vector2::new(0, 0));
        }

        if self.curr_x < LINE_SIZE {
            self.curr_x += 1;
        } else if self.curr_y < LINE_SIZE {
            self.curr_y += 1;
            self.curr_x = 1;
        } else {
            return None;
        }

        Some(Vector2::new(self.curr_x - 1, self.curr_y - 1))
    }
}

#[derive(Clone, Debug)]
pub struct Board {
    pieces: [Option<OwnedPiece>; LINE_SIZE.pow(2) as usize],
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for pos in BoardIter::new() {
            if let Some(piece) = self[pos] {
                write!(f, "{:}", piece)?;
            } else {
                write!(f, "")?;
            }
            if pos.x == 7 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl Index<Vector2<u8>> for Board {
    type Output = Option<OwnedPiece>;

    fn index(&self, index: Vector2<u8>) -> &Self::Output {
        debug_assert!((0..LINE_SIZE).contains(&index.x));
        debug_assert!((0..LINE_SIZE).contains(&index.y));

        &self.pieces[(index.y * LINE_SIZE + index.x) as usize]
    }
}

impl IndexMut<Vector2<u8>> for Board {
    fn index_mut(&mut self, index: Vector2<u8>) -> &mut Self::Output {
        debug_assert!((0..LINE_SIZE).contains(&index.x));
        debug_assert!((0..LINE_SIZE).contains(&index.y));

        &mut self.pieces[(index.y * LINE_SIZE + index.x) as usize]
    }
}

impl Board {
    const fn empty() -> Self {
        Self {
            pieces: [None; LINE_SIZE.pow(2) as usize],
        }
    }

    pub const fn new() -> Self {
        let mut out = Self::empty();

        out
    }
}
