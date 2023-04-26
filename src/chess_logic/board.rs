use cgmath::Vector2;
use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

use crate::chess_logic::{MoveDescriptor, OwnedPiece, Piece, Player};

pub const BOARD_SIZE: u8 = 8;
pub const WHITE_PAWN_START_ROW: u8 = 1;
pub const WHITE_PAWN_END_ROW: u8 = 7;
pub const BLACK_PAWN_START_ROW: u8 = 6;
pub const BLACK_PAWN_END_ROW: u8 = 0;

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

        if self.curr_x < BOARD_SIZE {
            self.curr_x += 1;
        } else if self.curr_y < BOARD_SIZE {
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
    pieces: [Option<OwnedPiece>; BOARD_SIZE.pow(2) as usize],
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "  abcdefgh  ")?;

        for pos in BoardIter::new() {
            let pos = Vector2::new(pos.x, (BOARD_SIZE - 1) - pos.y);
            if pos.x == 0 {
                write!(f, "{} ", pos.y + 1)?;
            }

            if let Some(piece) = self[pos] {
                write!(f, "{}", piece)?;
            } else {
                write!(f, "*")?;
            }

            if pos.x == BOARD_SIZE - 1 {
                write!(f, " {}", pos.y + 1)?;
                writeln!(f)?;
            }
        }

        write!(f, "  abcdefgh  ")?;

        Ok(())
    }
}

impl Index<Vector2<u8>> for Board {
    type Output = Option<OwnedPiece>;

    fn index(&self, index: Vector2<u8>) -> &Self::Output {
        debug_assert!((0..BOARD_SIZE).contains(&index.x));
        debug_assert!((0..BOARD_SIZE).contains(&index.y));

        &self.pieces[(index.y * BOARD_SIZE + index.x) as usize]
    }
}

impl IndexMut<Vector2<u8>> for Board {
    fn index_mut(&mut self, index: Vector2<u8>) -> &mut Self::Output {
        debug_assert!((0..BOARD_SIZE).contains(&index.x));
        debug_assert!((0..BOARD_SIZE).contains(&index.y));

        &mut self.pieces[(index.y * BOARD_SIZE + index.x) as usize]
    }
}

impl Board {
    const fn empty() -> Self {
        Self {
            pieces: [None; BOARD_SIZE.pow(2) as usize],
        }
    }

    pub fn new() -> Self {
        let mut out = Self::empty();

        BoardIter::new().for_each(|pos| {
            if pos == Vector2::new(0, 0) {
                out[pos] = Some(OwnedPiece::new(Player::White, Piece::Rook))
            } else if pos == Vector2::new(1, 0) {
                out[pos] = Some(OwnedPiece::new(Player::White, Piece::Knight))
            } else if pos == Vector2::new(2, 0) {
                out[pos] = Some(OwnedPiece::new(Player::White, Piece::Bishop))
            } else if pos == Vector2::new(3, 0) {
                out[pos] = Some(OwnedPiece::new(Player::White, Piece::Queen))
            } else if pos == Vector2::new(4, 0) {
                out[pos] = Some(OwnedPiece::new(Player::White, Piece::King))
            } else if pos == Vector2::new(5, 0) {
                out[pos] = Some(OwnedPiece::new(Player::White, Piece::Bishop))
            } else if pos == Vector2::new(6, 0) {
                out[pos] = Some(OwnedPiece::new(Player::White, Piece::Knight))
            } else if pos == Vector2::new(7, 0) {
                out[pos] = Some(OwnedPiece::new(Player::White, Piece::Rook))
            } else if pos == Vector2::new(0, 1) {
                out[pos] = Some(OwnedPiece::new(Player::White, Piece::Pawn))
            } else if pos == Vector2::new(1, 1) {
                out[pos] = Some(OwnedPiece::new(Player::White, Piece::Pawn))
            } else if pos == Vector2::new(2, 1) {
                out[pos] = Some(OwnedPiece::new(Player::White, Piece::Pawn))
            } else if pos == Vector2::new(3, 1) {
                out[pos] = Some(OwnedPiece::new(Player::White, Piece::Pawn))
            } else if pos == Vector2::new(4, 1) {
                out[pos] = Some(OwnedPiece::new(Player::White, Piece::Pawn))
            } else if pos == Vector2::new(5, 1) {
                out[pos] = Some(OwnedPiece::new(Player::White, Piece::Pawn))
            } else if pos == Vector2::new(6, 1) {
                out[pos] = Some(OwnedPiece::new(Player::White, Piece::Pawn))
            } else if pos == Vector2::new(7, 1) {
                out[pos] = Some(OwnedPiece::new(Player::White, Piece::Pawn))
            } else if pos == Vector2::new(0, 7) {
                out[pos] = Some(OwnedPiece::new(Player::Black, Piece::Knight))
            } else if pos == Vector2::new(1, 7) {
                out[pos] = Some(OwnedPiece::new(Player::Black, Piece::Bishop))
            } else if pos == Vector2::new(2, 7) {
                out[pos] = Some(OwnedPiece::new(Player::Black, Piece::Queen))
            } else if pos == Vector2::new(3, 7) {
                out[pos] = Some(OwnedPiece::new(Player::Black, Piece::King))
            } else if pos == Vector2::new(4, 7) {
                out[pos] = Some(OwnedPiece::new(Player::Black, Piece::Bishop))
            } else if pos == Vector2::new(5, 7) {
                out[pos] = Some(OwnedPiece::new(Player::Black, Piece::Knight))
            } else if pos == Vector2::new(6, 7) {
                out[pos] = Some(OwnedPiece::new(Player::Black, Piece::Rook))
            } else if pos == Vector2::new(7, 7) {
                out[pos] = Some(OwnedPiece::new(Player::Black, Piece::Pawn))
            } else if pos == Vector2::new(0, 6) {
                out[pos] = Some(OwnedPiece::new(Player::Black, Piece::Pawn))
            } else if pos == Vector2::new(1, 6) {
                out[pos] = Some(OwnedPiece::new(Player::Black, Piece::Pawn))
            } else if pos == Vector2::new(2, 6) {
                out[pos] = Some(OwnedPiece::new(Player::Black, Piece::Pawn))
            } else if pos == Vector2::new(3, 6) {
                out[pos] = Some(OwnedPiece::new(Player::Black, Piece::Pawn))
            } else if pos == Vector2::new(4, 6) {
                out[pos] = Some(OwnedPiece::new(Player::Black, Piece::Pawn))
            } else if pos == Vector2::new(5, 6) {
                out[pos] = Some(OwnedPiece::new(Player::Black, Piece::Pawn))
            } else if pos == Vector2::new(6, 6) {
                out[pos] = Some(OwnedPiece::new(Player::Black, Piece::Pawn))
            } else if pos == Vector2::new(7, 6) {
                out[pos] = Some(OwnedPiece::new(Player::Black, Piece::Pawn))
            }
        });

        out
    }

    pub fn is_move_valid(&self, move_command: MoveDescriptor) -> bool {
        // TODO: Implement
        true
    }

    pub fn do_move(&mut self, move_command: MoveDescriptor) {
        self[move_command.to] = self[move_command.from];
        self[move_command.from] = None
    }

    pub fn evaluate_bord(&self) -> BoardCondition {
        // TODO: Implement
        BoardCondition::Continue
    }
}

#[derive(Clone, Copy, Debug)]
pub enum BoardCondition {
    Win(Player),
    Draw,
    Continue,
}
