
use crate::model::{GameRequest, GameResponse, GameState, Deck};
use crate::response_builder::ResponseBuilder;
use crate::utils::calculate_hand_value;

pub trait GameService {
    fn start(&self, request: GameRequest) -> GameResponse;
    fn hit(&self, request: GameRequest) -> GameResponse;
    fn stand(&self, request:GameRequest) -> GameResponse;
}

pub struct GameServiceImpl {
    pub deck:Deck
}

impl GameService for GameServiceImpl {
    fn start(&self, request: GameRequest) -> GameResponse {
        let mut deck_to_use = self.deck.clone();
        let mut state = GameState::new();
        state.set_balance(request.state.clone().balance);
        state.set_stake(request.state.clone().stake);

        let first_player_card = deck_to_use.deal_card();
        let second_player_card = deck_to_use.deal_card();

        state.player_hand.push(first_player_card.unwrap());
        state.player_hand.push(second_player_card.unwrap());

        let first_dealer_card = deck_to_use.deal_card();
        state.dealer_hand.push(first_dealer_card.unwrap());

        let response_buildr:ResponseBuilder = ResponseBuilder::new();
        response_buildr.build_response(state)

    }

    fn hit(&self, request: GameRequest) -> GameResponse {
        let mut deck_to_use = self.deck.clone();
        let mut state = request.state.clone();
        let player_hand = &mut state.player_hand;
        let dealer_hand = &state.dealer_hand;
        let split_hand = &mut state.player_split_hand;

        let mut drawn_card;
        loop {
            drawn_card = deck_to_use.deal_card().unwrap();
            if !player_hand.contains(&drawn_card) && !dealer_hand.contains(&drawn_card) && !split_hand.contains(&drawn_card) {
                break;
            }
        }
        if request.hand_number.is_none() {
            return GameResponse {
                state: state,
                status: "falure".to_string(),
                message: "Request is missing hand_number parameter in the body!".to_string()
            }
        } else if request.hand_number.unwrap() == 0 {
            player_hand.push(drawn_card);
        } else {
            split_hand.push(drawn_card);
        }
        state.player_hand = player_hand.to_vec();
        state.player_split_hand = split_hand.to_vec();

        let response_buildr:ResponseBuilder = ResponseBuilder::new();
        response_buildr.build_response(state)
    }

    fn stand(&self, request: GameRequest) -> GameResponse {
        let mut deck_to_use = self.deck.clone();
        let mut state = request.state.clone();
        let player_hand = &state.player_hand;
        let mut dealer_hand = state.dealer_hand.clone();
        let split_hand = &state.player_split_hand;

        while calculate_hand_value(&dealer_hand) <= 17 {
            let mut drawn_card;
            loop {
                drawn_card = deck_to_use.deal_card().unwrap();
                if !player_hand.contains(&drawn_card) && !dealer_hand.contains(&drawn_card) && 
                    !split_hand.contains(&drawn_card) {
                    break;
                }
            }
            dealer_hand.push(drawn_card);
        }

        state.dealer_hand = dealer_hand;
        state.is_round_over = true;

        let response_buildr:ResponseBuilder = ResponseBuilder::new();
        response_buildr.build_response(state)
    }
}

