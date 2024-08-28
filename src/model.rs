use serde::{Serialize, Deserialize};
use rand::seq::SliceRandom; // For the `shuffle` method
use rand::thread_rng;    

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub struct Card {
    suit: Suit,
    rank: Rank,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
enum Suit {
    Hearts, Diamonds, Clubs, Spades,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum Rank {
    Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace
}

impl Rank {
    pub fn value(&self) -> i32 {
        match self {
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 10,
            Rank::Queen => 10,
            Rank::King => 10,
            Rank::Ace => 11,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Deck {
    pub cards: Vec<Card>
}


impl Deck {
    pub fn new() -> Self {
        let mut deck = Deck { cards: Vec::new() };
        deck.initialize_deck();
        deck.shuffle();
        deck
    }

    fn initialize_deck(&mut self) {
        for &suit in &[Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
            for &rank in &[Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Six, Rank::Seven,
                           Rank::Eight, Rank::Nine, Rank::Ten, Rank::Jack, Rank::Queen, Rank::King, Rank::Ace] {
                self.cards.push(Card { suit, rank });
            }
        }
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn deal_card(&mut self) -> Option<Card> {
        if self.is_deck_empty() {
            panic!("Cannot deal from an empty deck");
        }

        let card = self.cards.pop();
        self.shuffle();
        card
    }

    fn is_deck_empty(&self) -> bool {
        self.cards.is_empty()
    }
}

#[derive(Serialize, Deserialize)]
pub struct GameState {
    pub player_hand: Vec<Card>,
    pub dealer_hand: Vec<Card>,
    pub player_won: bool,
    pub dealer_won: bool,
    pub is_round_over: bool,
    pub balance: f64,
    pub stake: f64,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_creation() {
        let card = Card {
            suit: Suit::Hearts, 
            rank: Rank::King
        };

        assert_eq!(card.suit, Suit::Hearts);
        assert_eq!(card.rank, Rank::King);
        assert_eq!(card.rank.value(), 10);
    }

    #[test]
    fn test_deck_initialization() {
        let deck = Deck::new();
        assert_eq!(deck.cards.len(), 52);
    }


    #[test]
    fn test_deck_shuffle() {
        let mut deck = Deck::new();
        let deck_copy = deck.clone();

        deck.shuffle();

        assert_ne!(deck.cards, deck_copy.cards);
    }

    #[test]
    fn test_deal_card() {
        let mut deck = Deck::new();
        let card = deck.deal_card().unwrap();

        assert!(card.rank.value() > 0);
        assert_eq!(deck.cards.len(), 51);
    }

    #[test]
    #[should_panic(expected = "Cannot deal from an empty deck")]
    fn test_deal_from_empty_deck() {
        let mut deck = Deck {cards: Vec::new()};
        deck.deal_card();
    }




   
}