
use crate::model::{GameRequest, GameResponse, GameState, Deck};

pub trait GameService {
    fn start(&self, request: GameRequest) -> GameResponse;
}

pub struct GameServiceImpl {
    pub deck:Deck
}

impl GameService for GameServiceImpl {
    fn start(&self, request: GameRequest) -> GameResponse {
        let mut state = GameState::new();
        state.set_balance(request.state.clone().balance);
        state.set_stake(request.state.clone().stake);

        let first_player_card = self.deck.clone().deal_card();
        let second_player_card = self.deck.clone().deal_card();

        state.player_hand.push(first_player_card.unwrap());
        state.player_hand.push(second_player_card.unwrap());

        let first_dealer_card = self.deck.clone().deal_card();
        state.dealer_hand.push(first_dealer_card.unwrap());

        GameResponse {
            status: "success".to_string(),
            message: "Split successful!".to_string(),
            state: state,
        }
    }
}

