use crate::model::{ GameState, GameRequest, GameResponse};

pub struct GameController;

impl GameController {
    pub fn start(request: GameRequest) -> GameResponse {
        let mut state = request.state;
        GameResponse {
            status: "success".to_string(),
            message: "Game started!".to_string(),
            state: state,
        }
    }

    pub fn split(request: GameRequest) -> GameResponse {
        let mut state = request.state;
        GameResponse {
            status: "success".to_string(),
            message: "Split successful!".to_string(),
            state: state,
        }
    }

    pub fn double_stake(request: GameRequest) -> GameResponse {
        let mut state = request.state;
        GameResponse {
            status: "success".to_string(),
            message: "Doubled the stake successfully!".to_string(),
            state: state,
        }
    }

    pub fn stand(request: GameRequest) -> GameResponse {
        let mut state = request.state;
        GameResponse {
            status: "success".to_string(),
            message: "Standed!".to_string(),
            state: state,
        }
    }

    pub fn hit(request: GameRequest) -> GameResponse {
        let mut state = request.state;
        GameResponse {
            status: "success".to_string(),
            message: "Hit!".to_string(),
            state: state,
        }
    }
    
}