
use crate::model::{GameRequest, GameResponse, GameState, Deck, Card};
use crate::response_builder::ResponseBuilder;
use crate::utils::calculate_hand_value;

pub trait GameService {
    fn start(&self, request: GameRequest) -> GameResponse;
    fn hit(&self, request: GameRequest) -> GameResponse;
    fn stand(&self, request:GameRequest) -> GameResponse;
    fn split(&self, request:GameRequest) -> GameResponse;
    fn double(&self, request:GameRequest) -> GameResponse;
}

pub struct GameServiceImpl;

impl GameService for GameServiceImpl {
    fn start(&self, request: GameRequest) -> GameResponse {
        let mut deck = Deck::new();
        let mut state = GameState::new();
        state.balance = state.clone().balance;
        state.stake = request.stake.unwrap();

        let first_player_card = deck.deal_card().unwrap();
        let second_player_card = deck.deal_card().unwrap();

        state.player_hand.push(first_player_card);
        state.player_hand.push(second_player_card);

        let first_dealer_card = deck.deal_card().unwrap();
        state.dealer_hand.push(first_dealer_card);

        let response_buildr:ResponseBuilder = ResponseBuilder::new();
        response_buildr.build_response(state)

    }

    fn hit(&self, request: GameRequest) -> GameResponse {
        let mut state = request.state.clone();
        let player_hand = &mut state.player_hand;
        let split_hand = &mut state.player_split_hand;

        let drawn_card = deal_non_dealt_card(request.state.clone().cards_dealt);
        state.cards_dealt.push(drawn_card);
        
        if request.hand_number.unwrap() == 0 {
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
        let mut state = request.state.clone();
        let mut dealer_hand = state.dealer_hand.clone();

        while calculate_hand_value(&dealer_hand) <= 17 {
            let drawn_card = deal_non_dealt_card(state.clone().cards_dealt);
            state.cards_dealt.push(drawn_card);
            dealer_hand.push(drawn_card);
        }

        state.dealer_hand = dealer_hand;
        state.is_round_over = true;

        let response_buildr:ResponseBuilder = ResponseBuilder::new();
        response_buildr.build_response(state)
    }

    fn split(&self, request:GameRequest) -> GameResponse {
        let mut state = request.state.clone();

        state.additional_stake = request.additional_stake.unwrap();

        let mut player_hand = state.player_hand.clone();
        let mut split_hand: Vec<Card> = Vec::new();
        
        split_hand.push(*player_hand.clone().get(1).unwrap());
        player_hand.remove(1);
        
        state.player_hand = player_hand;
        state.player_split_hand = split_hand;

        let response_buildr:ResponseBuilder = ResponseBuilder::new();
        response_buildr.build_response(state)
    }

    fn double(&self, mut request:GameRequest) -> GameResponse {
        let mut state = request.state.clone();
        let mut player_hand = state.player_hand.clone();
    
        let drawn_card = deal_non_dealt_card(state.clone().cards_dealt);
        
        state.cards_dealt.push(drawn_card);
        player_hand.push(drawn_card);
        state.player_hand = player_hand;

        state.is_stake_doubled = true;
        request.state = state;
        
        self.stand(request)
    }
}

fn deal_non_dealt_card(cards_dealt:Vec<Card>) -> Card{
    let mut deck = Deck::new();
    let mut drawn_card;
    loop {
        drawn_card = deck.deal_card().unwrap();
        if !cards_dealt.contains(&drawn_card) {
            break;
        }
    }
    drawn_card
}

