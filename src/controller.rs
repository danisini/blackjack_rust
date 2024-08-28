use crate::model::{ GameState, GameRequest, GameResponse};

pub struct GameController;

impl GameController {
    pub fn start(request: GameRequest) -> GameResponse {
        let game_state = GameState {
            player_hand: Vec::new(),
            dealer_hand: Vec::new(),
            player_won: false,
            dealer_won: false,
            is_round_over: false,
            balance: 0.0,
            stake: 0.0,
        };
        GameResponse {
            status: "success".to_string(),
            message: "Game started".to_string(),
            state: game_state,
        }
    }
}