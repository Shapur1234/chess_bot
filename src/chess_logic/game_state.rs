use super::{Board, Player};

pub struct GameState {
    board: Board,
    turn: Player,
}

impl GameState {
    pub fn new(starting_player: Player) -> Self {
        Self {
            board: Board::new(),
            turn: starting_player,
        }
    }
}
