use std::{fmt::Display, ops::Neg};

use crate::chess_logic::{Board, BoardCondition, MoveDescriptor, Player};

#[derive(Clone, Copy, Debug)]
pub enum StateUpdate {
    Continue,
    InvalidMove,
    Draw,
    Win(Player),
}

#[derive(Clone, Debug)]
pub struct GameState {
    board: Board,
    turn: Player,
}

impl Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}'s turn", self.turn)?;
        writeln!(f, "{}", self.board)?;

        Ok(())
    }
}

impl GameState {
    pub fn new(starting_player: impl Into<Player>) -> Self {
        Self {
            board: Board::new(),
            turn: starting_player.into(),
        }
    }

    pub fn play(&mut self, move_command: impl Into<MoveDescriptor>) -> StateUpdate {
        let move_command = move_command.into();

        if self.board.is_move_valid(move_command) {
            self.board.do_move(move_command);
            match self.board.evaluate_bord() {
                BoardCondition::Win(player) => StateUpdate::Win(player),
                BoardCondition::Draw => StateUpdate::Draw,
                BoardCondition::Continue => {
                    self.turn = self.turn.neg();
                    StateUpdate::Continue
                }
            }
        } else {
            StateUpdate::InvalidMove
        }
    }
}
