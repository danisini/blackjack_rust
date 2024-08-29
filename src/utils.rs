use std::char::MAX;

use crate::model::{Card, Suit, Rank};
const MAX_NUMBER_OF_POINTS: i32 = 21;

pub fn calculate_hand_value(hand: &[Card]) -> i32 {
    let mut value = 0;
    let mut number_of_aces = 0;

    for card in hand {
        let rank_value = card.rank.value();
        if card.rank == Rank::Ace {
            number_of_aces += 1;
        }
        value += rank_value;
    }


    while value > MAX_NUMBER_OF_POINTS && number_of_aces > 0 {
        value -= 10;
        number_of_aces -= 1;
    }

    value
}

pub fn is_participant_busted(hand: &[Card]) -> bool {
    calculate_hand_value(hand) > MAX_NUMBER_OF_POINTS
}

pub fn has_player_won(player_hand: &[Card], dealer_hand: &[Card]) -> bool {
    let player_value = calculate_hand_value(player_hand);
    let dealer_value = calculate_hand_value(dealer_hand);

    if player_hand.is_empty() {
        return false;
    }
    
    if is_participant_busted(player_hand) {
        return false;
    }

    if is_participant_busted(dealer_hand) {
        return true;
    }

    if player_value > dealer_value {
        return true;
    }

    if player_value == MAX_NUMBER_OF_POINTS && dealer_value != MAX_NUMBER_OF_POINTS {
        return true;
    }

    false
}

pub fn has_dealer_won(player_hand: &[Card], dealer_hand: &[Card]) -> bool {
    !has_player_won(player_hand, dealer_hand) && calculate_hand_value(player_hand) != calculate_hand_value(dealer_hand)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn test_calculate_hand_value() {
        let mut player_hand = Vec::new();
        player_hand.push(Card { suit: Suit::Hearts, rank: Rank::Ace});
        player_hand.push(Card { suit: Suit::Hearts, rank: Rank::Ace});

        let hand_value = calculate_hand_value(&player_hand);
        assert_eq!(hand_value, 12);
    }

    #[test]
    fn test_is_participant_busted() {
        let mut player_hand = Vec::new();
        player_hand.push(Card { suit: Suit::Hearts, rank: Rank::King});
        player_hand.push(Card { suit: Suit::Spades, rank: Rank::King});
        player_hand.push(Card { suit: Suit::Hearts, rank: Rank::Ten});

        assert!(is_participant_busted(&player_hand));
    }


    #[test]
    fn test_has_player_won() {
        let mut player_hand = Vec::new();
        player_hand.push(Card {suit: Suit::Hearts, rank: Rank::Ten});
        player_hand.push(Card {suit: Suit::Hearts, rank: Rank::Ten});


        let mut dealer_hand = Vec::new();
        dealer_hand.push(Card {suit: Suit::Hearts, rank: Rank::Ten});
        dealer_hand.push(Card {suit: Suit::Hearts, rank: Rank::Nine});

        assert!(has_player_won(&player_hand, &dealer_hand));           
    }
    #[test]
    fn test_has_dealer_won() {
        let mut player_hand = Vec::new();
        player_hand.push(Card {suit: Suit::Hearts, rank: Rank::Ten});
        player_hand.push(Card {suit: Suit::Hearts, rank: Rank::Nine});


        let mut dealer_hand = Vec::new();
        dealer_hand.push(Card {suit: Suit::Hearts, rank: Rank::Ten});
        dealer_hand.push(Card {suit: Suit::Hearts, rank: Rank::Ten});

        assert!(has_dealer_won(&player_hand, &dealer_hand));   
    }
}