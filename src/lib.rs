pub mod rank;
pub mod suit;

pub use rank::*;
pub use suit::*;

use enum_iterator::IntoEnumIterator;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Card {
        Card { rank, suit }
    }

    pub fn get_rank(&self) -> Rank {
        self.rank
    }

    pub fn get_suit(&self) -> Suit {
        self.suit
    }

    pub fn get_value(&self) -> u8 {
        Rank::get_value(&self.rank)
    }

    pub fn get_full_card_name(&self) -> String {
        format!("{} of {}", self.rank, self.suit)
    }
}

pub fn deck_52() -> Vec<Card> {
    let mut deck = Vec::new();
    for rank in Rank::into_enum_iter() {
        for suit in Suit::into_enum_iter() {
            deck.push(Card::new(rank, suit));
        }
    }
    deck
}

pub fn deck_32() -> Vec<Card> {
    let mut deck = Vec::new();
    for rank in Rank::into_enum_iter().skip(5) {
        for suit in Suit::into_enum_iter() {
            deck.push(Card::new(rank, suit));
        }
    }
    deck
}

pub fn hello_world() {
    println!("Hello from Rust Cards!!");
}