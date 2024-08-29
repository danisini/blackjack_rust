
use crate::model::{GameRequest, GameResponse, GameState, Deck};
use crate::response_builder::{self, ResponseBuilder};

pub trait GameService {
    fn start(&self, request: GameRequest) -> GameResponse;
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
}

