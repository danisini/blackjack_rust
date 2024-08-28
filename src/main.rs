pub mod model;

fn main() {
    println!("Welcome to Blackjack!");

    // Example of using something from model.rs
    let deck = model::Deck::new();
    println!("Deck initialized with {} cards.", deck.cards.len());
}