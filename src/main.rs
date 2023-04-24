use std::io::{self, Write};

use crate::chess_logic::{GameState, MoveDescriptor, Player, StateUpdate};

mod chess_logic;
mod parameters;

fn main() -> io::Result<()> {
    let mut game_state = GameState::new(Player::White);

    loop {
        let mut user_input = String::new();
        let stdin = io::stdin();

        // let parameters =
        //     confy::load_path::<Parameters>(default_parameters_location()).unwrap_or_default();
        // confy::store_path(default_parameters_location(), parameters).ok();

        print!("{}[2J", 27 as char);
        println!("{game_state:}");

        print!("Your move: ");
        io::stdout().flush()?;

        stdin.read_line(&mut user_input)?;

        if let Ok(move_command) = MoveDescriptor::try_from(user_input) {
            match game_state.play(move_command) {
                StateUpdate::Draw => {
                    println!("Draw");
                    break;
                }
                StateUpdate::Win(player) => {
                    println!("{} won", player);
                    break;
                }
                StateUpdate::InvalidMove | StateUpdate::Continue => {}
            }
        } else {
            continue;
        }
    }

    Ok(())
}
