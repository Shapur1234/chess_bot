use cgmath::Vector2;

use crate::chess_logic::BOARD_SIZE;

// TODO: Check castle, ompasant

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct MoveDescriptor {
    pub from: Vector2<u8>,
    pub to: Vector2<u8>,
}

impl MoveDescriptor {
    pub fn new(from: impl Into<Vector2<u8>>, to: impl Into<Vector2<u8>>) -> Option<Self> {
        let (from, to) = (from.into(), to.into());

        if fits_on_board(from, to) {
            Some(Self { from, to })
        } else {
            None
        }
    }
}

impl TryFrom<String> for MoveDescriptor {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.trim();

        if value.len() == 4 {
            let (mut from, mut to) = (Vector2::new(0, 0), Vector2::new(0, 0));

            for (index, char) in value.char_indices() {
                if (index % 2 == 0 && char.is_alphabetic()) | (index % 2 != 0 && char.is_numeric())
                {
                    if let Ok(num) = char_to_num(char) {
                        match index {
                            0 => from.x = num,
                            1 => from.y = num,
                            2 => to.x = num,
                            3 => to.y = num,
                            _ => unreachable!(),
                        }
                    } else {
                        return Err(());
                    }
                } else {
                    return Err(());
                }
            }

            Ok(MoveDescriptor::new(from, to).unwrap())
        } else {
            Err(())
        }
    }
}

const fn fits_on_board(from: Vector2<u8>, to: Vector2<u8>) -> bool {
    from.x < BOARD_SIZE && from.y < BOARD_SIZE && to.x < BOARD_SIZE && to.y < BOARD_SIZE
}

const fn char_to_num(char: char) -> Result<u8, ()> {
    match char {
        'a' | '1' => Ok(0),
        'b' | '2' => Ok(1),
        'c' | '3' => Ok(2),
        'd' | '4' => Ok(3),
        'e' | '5' => Ok(4),
        'f' | '6' => Ok(5),
        'g' | '7' => Ok(6),
        'h' | '8' => Ok(7),
        _ => Err(()),
    }
}
