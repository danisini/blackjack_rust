use crate::model::{GameResponse, GameState, Card};
use crate::utils::{is_participant_busted, has_player_won, has_dealer_won};

pub struct ResponseBuilder;


impl ResponseBuilder {
    pub fn new() -> Self {
        Self
    }

    pub fn build_response(&self, state:GameState) -> GameResponse {
        let response_state = self.update_state(state);
        GameResponse {
            state: response_state,
            status: "success".to_string(),
            message: "Successful operation!".to_string()
        }
    }

    fn update_state(&self, mut state:GameState) -> GameState {
        let mut response_state = GameState::new();
        let player_hand = state.clone().player_hand;
        let dealer_hand = state.clone().dealer_hand;
        let split_hand = state.clone().player_split_hand;

        let mut possible_actions: Vec<String> = Vec::new();

        if self.can_game_continue(&player_hand, &dealer_hand, state.clone().is_round_over) {
            if self.is_split_possible(&player_hand) {
                possible_actions.push("/split".to_string());
            }

            possible_actions.push("/double".to_string());
            possible_actions.push("/stand".to_string());
            possible_actions.push("/hit".to_string());
            response_state.stake = state.clone().stake;

        } else {
            possible_actions.push("/start".to_string());
            let has_first_hand_player_won = has_player_won(&player_hand, &dealer_hand);
            let has_dealer_won = has_dealer_won(&player_hand, &dealer_hand);
            let has_player_split_won = has_player_won(&split_hand, &dealer_hand);
            let has_player_won = has_first_hand_player_won || has_player_split_won;

            if has_player_won {
                response_state = self.calculate_win_amount(state.clone(), has_first_hand_player_won, has_player_split_won);
            } else if has_dealer_won {
                response_state = self.calculate_lost_amount(state.clone());
            }
            response_state.has_player_won = has_player_won.clone();
            response_state.has_dealer_won = has_dealer_won.clone();
            response_state.is_round_over = has_dealer_won || has_player_won;
        }

        response_state.additional_stake = state.clone().additional_stake;
        response_state.balance = state.balance;
        response_state.possible_actions = possible_actions;
        response_state.player_split_hand = split_hand;
        response_state.dealer_hand = dealer_hand;
        response_state.player_hand = player_hand;
        response_state.cards_dealt = state.cards_dealt;

        response_state
    }

    fn calculate_lost_amount(&self, mut state: GameState) -> GameState {
        let mut lost_amount = state.stake + state.additional_stake;
        if state.is_stake_doubled {
            lost_amount *= 2.0;
        }

        state.balance -= lost_amount;
        state
    }


    fn calculate_win_amount(&self, mut state: GameState, has_first_hand_player_won: bool, has_player_split_won: bool) -> GameState {
        let mut win_amount = 0.0;
        if has_first_hand_player_won {
            win_amount += state.stake;
        }
        if has_player_split_won {
            win_amount += state.additional_stake;
        }
        if state.is_stake_doubled {
            win_amount *= 2.0;
        }

        state.win_amount = win_amount;
        state.balance += win_amount;

        state
    }

    fn is_split_possible(&self, player_hand: &Vec<Card>) -> bool {
        player_hand.len() == 2 && player_hand[0].rank == player_hand[1].rank
    }

    fn can_game_continue(&self, player_hand: &Vec<Card>, dealer_hand: &Vec<Card>, is_round_over: bool) -> bool {
        !is_round_over && !is_participant_busted(player_hand) && !is_participant_busted(dealer_hand)
    }

}