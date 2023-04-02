use ansi_term::Colour::{Green, Red};
use cgmath::Vector2;
use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

const BOARD_WIDTH: u8 = 8;

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
        match self.0 {
            Player::White => write!(f, "{}", Green.paint(self.1.to_string())),
            Player::Black => write!(f, "{}", Red.paint(self.1.to_string())),
        }
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

        if self.curr_x < BOARD_WIDTH {
            self.curr_x += 1;
        } else if self.curr_y < BOARD_WIDTH {
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
    pieces: [Option<OwnedPiece>; BOARD_WIDTH.pow(2) as usize],
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for pos in BoardIter::new() {
            let pos = Vector2::new(pos.x, (BOARD_WIDTH - 1) - pos.y);
            if let Some(piece) = self[pos] {
                write!(f, "{}", piece)?;
            } else {
                write!(f, "-")?;
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
        debug_assert!((0..BOARD_WIDTH).contains(&index.x));
        debug_assert!((0..BOARD_WIDTH).contains(&index.y));

        &self.pieces[(index.y * BOARD_WIDTH + index.x) as usize]
    }
}

impl IndexMut<Vector2<u8>> for Board {
    fn index_mut(&mut self, index: Vector2<u8>) -> &mut Self::Output {
        debug_assert!((0..BOARD_WIDTH).contains(&index.x));
        debug_assert!((0..BOARD_WIDTH).contains(&index.y));

        &mut self.pieces[(index.y * BOARD_WIDTH + index.x) as usize]
    }
}

impl Board {
    const fn empty() -> Self {
        Self {
            pieces: [None; BOARD_WIDTH.pow(2) as usize],
        }
    }

    pub fn new() -> Self {
        let mut out = Self::empty();

        for pos in BoardIter::new() {
            if pos == Vector2::new(0, 0) {
                out[pos] = Some(OwnedPiece(Player::White, Piece::Rook))
            } else if pos == Vector2::new(1, 0) {
                out[pos] = Some(OwnedPiece(Player::White, Piece::Knight))
            } else if pos == Vector2::new(2, 0) {
                out[pos] = Some(OwnedPiece(Player::White, Piece::Bishop))
            } else if pos == Vector2::new(3, 0) {
                out[pos] = Some(OwnedPiece(Player::White, Piece::Queen))
            } else if pos == Vector2::new(4, 0) {
                out[pos] = Some(OwnedPiece(Player::White, Piece::King))
            } else if pos == Vector2::new(5, 0) {
                out[pos] = Some(OwnedPiece(Player::White, Piece::Bishop))
            } else if pos == Vector2::new(6, 0) {
                out[pos] = Some(OwnedPiece(Player::White, Piece::Knight))
            } else if pos == Vector2::new(7, 0) {
                out[pos] = Some(OwnedPiece(Player::White, Piece::Rook))
            } else if pos == Vector2::new(0, 1) {
                out[pos] = Some(OwnedPiece(Player::White, Piece::Pawn))
            } else if pos == Vector2::new(1, 1) {
                out[pos] = Some(OwnedPiece(Player::White, Piece::Pawn))
            } else if pos == Vector2::new(2, 1) {
                out[pos] = Some(OwnedPiece(Player::White, Piece::Pawn))
            } else if pos == Vector2::new(3, 1) {
                out[pos] = Some(OwnedPiece(Player::White, Piece::Pawn))
            } else if pos == Vector2::new(4, 1) {
                out[pos] = Some(OwnedPiece(Player::White, Piece::Pawn))
            } else if pos == Vector2::new(5, 1) {
                out[pos] = Some(OwnedPiece(Player::White, Piece::Pawn))
            } else if pos == Vector2::new(6, 1) {
                out[pos] = Some(OwnedPiece(Player::White, Piece::Pawn))
            } else if pos == Vector2::new(7, 1) {
                out[pos] = Some(OwnedPiece(Player::White, Piece::Pawn))
            } else if pos == Vector2::new(0, 7) {
                out[pos] = Some(OwnedPiece(Player::Black, Piece::Knight))
            } else if pos == Vector2::new(1, 7) {
                out[pos] = Some(OwnedPiece(Player::Black, Piece::Bishop))
            } else if pos == Vector2::new(2, 7) {
                out[pos] = Some(OwnedPiece(Player::Black, Piece::Queen))
            } else if pos == Vector2::new(3, 7) {
                out[pos] = Some(OwnedPiece(Player::Black, Piece::King))
            } else if pos == Vector2::new(4, 7) {
                out[pos] = Some(OwnedPiece(Player::Black, Piece::Bishop))
            } else if pos == Vector2::new(5, 7) {
                out[pos] = Some(OwnedPiece(Player::Black, Piece::Knight))
            } else if pos == Vector2::new(6, 7) {
                out[pos] = Some(OwnedPiece(Player::Black, Piece::Rook))
            } else if pos == Vector2::new(7, 7) {
                out[pos] = Some(OwnedPiece(Player::Black, Piece::Pawn))
            } else if pos == Vector2::new(0, 6) {
                out[pos] = Some(OwnedPiece(Player::Black, Piece::Pawn))
            } else if pos == Vector2::new(1, 6) {
                out[pos] = Some(OwnedPiece(Player::Black, Piece::Pawn))
            } else if pos == Vector2::new(2, 6) {
                out[pos] = Some(OwnedPiece(Player::Black, Piece::Pawn))
            } else if pos == Vector2::new(3, 6) {
                out[pos] = Some(OwnedPiece(Player::Black, Piece::Pawn))
            } else if pos == Vector2::new(4, 6) {
                out[pos] = Some(OwnedPiece(Player::Black, Piece::Pawn))
            } else if pos == Vector2::new(5, 6) {
                out[pos] = Some(OwnedPiece(Player::Black, Piece::Pawn))
            } else if pos == Vector2::new(6, 6) {
                out[pos] = Some(OwnedPiece(Player::Black, Piece::Pawn))
            } else if pos == Vector2::new(7, 6) {
                out[pos] = Some(OwnedPiece(Player::Black, Piece::Pawn))
            }
        }

        out
    }
}
