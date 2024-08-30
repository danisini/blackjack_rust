
use crate::model:: { GameState, GameRequest, GameResponse};
use crate::game_service::{GameService, GameServiceImpl};

pub struct GameController;

impl GameController {
    pub fn start(request: GameRequest) -> GameResponse {
        let state = request.clone().state;
        let mut is_valid = ValidatorImpl::is_action_valid(&state, "/start");
        is_valid = is_valid && ValidatorImpl::has_stake(request.clone());

        if !is_valid {
            GameResponse {
                status: "failure".to_string(),
                message: "Request not valid!".to_string(),
                state: request.clone().state
            }
        } else {
            let service = GameServiceImpl {};
            service.start(request)
        }
    }

    pub fn split(request: GameRequest) -> GameResponse {
        let state = request.clone().state;
        let mut is_valid = ValidatorImpl::is_action_valid(&state, "/split");
        is_valid = is_valid && ValidatorImpl::has_additional_stake(request.clone());
        if !is_valid {
            GameResponse {
                status: "failure".to_string(),
                message: "Request not valid!".to_string(),
                state: request.clone().state
            }
        } else {
            let service = GameServiceImpl {};
            service.split(request)
        }
    }

    pub fn double_stake(request: GameRequest) -> GameResponse {
        let state = request.clone().state;
        let mut is_valid = ValidatorImpl::is_action_valid(&state, "/double");
        is_valid = is_valid && ValidatorImpl::has_enough_balance(&state, state.additional_stake + state.stake);
        
        if !is_valid {
            GameResponse {
                status: "failure".to_string(),
                message: "Request not valid!".to_string(),
                state: request.clone().state
            }
        } else {
            let service = GameServiceImpl {};
            service.double(request)
        }
    }

    pub fn stand(request: GameRequest) -> GameResponse {
        let state = request.clone().state;
        let is_valid = ValidatorImpl::is_action_valid(&state, "/stand");
        if !is_valid {
            GameResponse {
                status: "failure".to_string(),
                message: "Request not valid!".to_string(),
                state: request.clone().state
            }
        } else {
            let service = GameServiceImpl {};
            service.stand(request)
        }
    }

    pub fn hit(request: GameRequest) -> GameResponse {
        let state = request.clone().state;
        let mut is_valid = ValidatorImpl::is_action_valid(&state, "/hit");
        is_valid = is_valid && ValidatorImpl::has_enough_balance(&state, state.clone().stake);
        is_valid = is_valid && ValidatorImpl::has_hand_number(request.clone());
        if !is_valid {
            GameResponse {
                status: "failure".to_string(),
                message: "Request not valid!".to_string(),
                state: request.clone().state
            }
        } else {
            let service = GameServiceImpl {};
            service.hit(request)
        }
    }
    
}

pub trait Validator {
    fn has_enough_balance(game_state: &GameState, stake: f64) -> bool;
    fn is_action_valid(game_state: &GameState, action: &str) -> bool;
    fn has_hand_number(request:GameRequest) -> bool;
    fn has_stake(request:GameRequest) -> bool;
    fn has_additional_stake(request:GameRequest) -> bool;
}

pub struct ValidatorImpl;

impl Validator for ValidatorImpl {
    fn has_enough_balance(game_state: &GameState, stake: f64) -> bool {
        game_state.balance >= stake
    }

    fn is_action_valid(game_state: &GameState, action: &str) -> bool {
        game_state.possible_actions.contains(&action.to_string())
    }

    fn has_hand_number(request:GameRequest) -> bool {
        request.hand_number.is_some()
    }

    fn has_additional_stake(request:GameRequest) -> bool {
        request.additional_stake.is_some()
    }

    fn has_stake(request:GameRequest) -> bool {
        request.stake.is_some()
    }
}

#[cfg(test)]
mod controller_tests {
    use super::*;
    use crate::model::{GameState, GameRequest, Rank, Suit, Card};
    
    fn create_request(state: GameState) -> GameRequest {
        GameRequest {
            state: state,
            hand_number: None,
            stake: None,
            additional_stake: None,
        }
    }

    fn create_state() -> GameState {
        GameState {
            win_amount:0.0,
            cards_dealt: vec![],
            has_player_won:false,
            has_dealer_won:false,
            is_round_over:false,
            is_stake_doubled:false,
            balance: 100.0,
            stake: 1.0,
            additional_stake: 0.0,
            player_hand: vec![],
            dealer_hand: vec![],
            player_split_hand: vec![],
            possible_actions: vec![],
        }
    }
 
    #[test]
    fn test_start_valid_request() {
        let mut state = create_state();
        state.possible_actions.push("/start".to_string());
        let mut request = create_request(state.clone());
        request.stake = Some(1.0);

        let response = GameController::start(request);

        assert_eq!(response.status, "success");
        assert_eq!(response.message, "Successful operation!");
    }

    #[test]
    fn test_start_no_stake() {
        let mut state = create_state();
        state.possible_actions.push("/start".to_string());
        let request = create_request(state.clone());

        let response = GameController::start(request);

        assert_eq!(response.status, "failure");
        assert_eq!(response.message, "Request not valid!");
    }

    #[test]
    fn test_start_not_possible_operation() {
        let state = create_state();
        let mut request = create_request(state.clone());
        request.stake = Some(1.0);

        let response = GameController::start(request);

        assert_eq!(response.status, "failure");
        assert_eq!(response.message, "Request not valid!");
    }

    #[test]
    fn test_split_valid_request() {
        let mut state = create_state();
        state.possible_actions.push("/split".to_string());
        let card = Card{ suit:Suit::Clubs, rank:Rank::Ace};

        state.player_hand.push(card);
        state.player_hand.push(card);
        let mut request = create_request(state);
        request.additional_stake = Some(2.0);

        let response = GameController::split(request);

        assert_eq!(response.status, "success");
        assert_eq!(response.message, "Successful operation!");
        assert!(response.state.player_split_hand.len() > 0);
        assert!(response.state.additional_stake > 0.0);
        assert!(!response.state.is_round_over);
    }

    #[test]
    fn test_split_not_possible_action() {
        let state = create_state();
        let mut request = create_request(state);
        request.additional_stake = Some(2.0);

        let response = GameController::split(request);

        assert_eq!(response.status, "failure");
        assert_eq!(response.message, "Request not valid!");
    }

    #[test]
    fn test_split_no_additional_stake() {
        let mut state = create_state();
        state.possible_actions.push("/split".to_string());
        let request = create_request(state);

        let response = GameController::split(request);

        assert_eq!(response.status, "failure");
        assert_eq!(response.message, "Request not valid!");
    }

    #[test]
    fn test_double_valid_request() {
        let mut state = create_state();
        state.possible_actions.push("/double".to_string());
        let card = Card{ suit:Suit::Clubs, rank:Rank::Ace};
        state.player_hand.push(card);
        state.player_hand.push(card);

        let request = create_request(state);
        let response = GameController::double_stake(request);

        assert_eq!(response.status, "success");
        assert_eq!(response.message, "Successful operation!");
        assert!(response.state.possible_actions.len() == 1)
    }

    #[test]
    fn test_double_not_possible_action() {
        let state = create_state();
        let request = create_request(state);

        let response = GameController::double_stake(request);

        assert_eq!(response.status, "failure");
        assert_eq!(response.message, "Request not valid!");
    }

    #[test]
    fn test_stand_valid_request() {
        let mut state = create_state();
        state.possible_actions.push("/stand".to_string());
        let card = Card{ suit:Suit::Clubs, rank:Rank::Ace};
        state.player_hand.push(card);
        state.player_hand.push(card);

        let request = create_request(state);
        let response = GameController::stand(request);

        assert_eq!(response.status, "success");
        assert_eq!(response.message, "Successful operation!");
        assert!(response.state.is_round_over);
        assert!(response.state.possible_actions.len() == 1)
    }

    #[test]
    fn test_stand_not_possible_action() {
        let state = create_state();
        let request = create_request(state);

        let response = GameController::stand(request);

        assert_eq!(response.status, "failure");
        assert_eq!(response.message, "Request not valid!");
    }


    #[test]
    fn test_hit_valid_request() {
        let mut state = create_state();
        state.possible_actions.push("/hit".to_string());
        let card = Card{ suit:Suit::Clubs, rank:Rank::Four};
        state.player_hand.push(card);
        state.player_hand.push(card);

        let mut request = create_request(state);
        request.hand_number = Some(0);
        let response = GameController::hit(request);

        assert_eq!(response.status, "success");
        assert_eq!(response.message, "Successful operation!");
    }

    #[test]
    fn test_hit_not_possible_action() {
        let state = create_state();
        let request = create_request(state);

        let response = GameController::hit(request);

        assert_eq!(response.status, "failure");
        assert_eq!(response.message, "Request not valid!");
    }

    #[test]
    fn test_hit_not_hand_number() {
        let mut state = create_state();
        state.possible_actions.push("/hit".to_string());

        let request = create_request(state);
        
        let response = GameController::hit(request);

        assert_eq!(response.status, "failure");
        assert_eq!(response.message, "Request not valid!");
    }
}