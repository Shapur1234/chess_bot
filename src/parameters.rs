use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

lazy_static! {
    pub static ref PARAMETERS: Parameters =
        confy::load_path(default_parameters_location()).unwrap_or_default();
}

#[derive(Serialize, Deserialize)]
pub struct Parameters {
    pub pawn_value: f32,
    pub bishop_value: f32,
    pub knight_value: f32,
    pub rook_value: f32,
    pub queen_value: f32,
    pub king_value: f32,
}

impl Default for Parameters {
    fn default() -> Self {
        Self {
            pawn_value: 1.0,
            bishop_value: 3.0,
            knight_value: 3.0,
            rook_value: 5.0,
            queen_value: 9.0,
            king_value: f32::MAX,
        }
    }
}

pub fn default_parameters_location() -> String {
    "./parameters.toml".to_string()
}
