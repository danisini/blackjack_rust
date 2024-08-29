
use crate::model:: { GameState, GameRequest, GameResponse, Deck};
use crate::game_service::{GameService, GameServiceImpl};

pub struct GameController;

impl GameController {
    pub fn start(request: GameRequest) -> GameResponse {
        let mut state = request.clone().state;
        let is_valid = ValidatorImpl::is_action_valid(&state, "/start");
        if !is_valid {
            GameResponse {
                status: "failure".to_string(),
                message: "Method not allowed!".to_string(),
                state: request.clone().state
            }
        } else {
            let service = GameServiceImpl {
                deck: Deck::new()
            };

            service.start(request)
        }
    }

    pub fn split(request: GameRequest) -> GameResponse {
        let mut state = request.clone().state;
        let is_valid = ValidatorImpl::is_action_valid(&state, "/split");
        
        if !is_valid {
            GameResponse {
                status: "failure".to_string(),
                message: "Method not allowed!".to_string(),
                state: request.clone().state
            }
        } else {
            GameResponse {
                status: "success".to_string(),
                message: "Split successful!".to_string(),
                state: state,
            }
        }
    }

    pub fn double_stake(request: GameRequest) -> GameResponse {
        let mut state = request.clone().state;
        let is_valid = ValidatorImpl::is_action_valid(&state, "/double");
        if !is_valid {
            GameResponse {
                status: "failure".to_string(),
                message: "Method not allowed!".to_string(),
                state: request.clone().state
            }
        } else {
            GameResponse {
            status: "success".to_string(),
            message: "Doubled the stake successfully!".to_string(),
            state: state,
            }
        }
    }

    pub fn stand(request: GameRequest) -> GameResponse {
        let mut state = request.clone().state;
        let is_valid = ValidatorImpl::is_action_valid(&state, "/stand");
        if !is_valid {
            GameResponse {
                status: "failure".to_string(),
                message: "Method not allowed!".to_string(),
                state: request.clone().state
            }
        } else {
            let service = GameServiceImpl {
                deck: Deck::new()
            };

            service.stand(request)
        }
    }

    pub fn hit(request: GameRequest) -> GameResponse {
        let mut state = request.clone().state;
        let mut is_valid = ValidatorImpl::is_action_valid(&state, "/hit");
        is_valid = is_valid && ValidatorImpl::has_enough_balance(&state, state.clone().stake);
        if !is_valid {
            GameResponse {
                status: "failure".to_string(),
                message: "Method not allowed!".to_string(),
                state: request.clone().state
            }
        } else {
            let service = GameServiceImpl {
                deck: Deck::new()
            };

            service.hit(request)
        }
    }
    
}

pub trait Validator {
    fn has_enough_balance(game_state: &GameState, stake: f64) -> bool;
    fn is_action_valid(game_state: &GameState, action: &str) -> bool;
}

pub struct ValidatorImpl;

impl Validator for ValidatorImpl {
    fn has_enough_balance(game_state: &GameState, stake: f64) -> bool {
        game_state.balance >= stake
    }

    fn is_action_valid(game_state: &GameState, action: &str) -> bool {
        game_state.possible_actions.contains(&action.to_string())
    }
}