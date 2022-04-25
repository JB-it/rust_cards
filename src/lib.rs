use enum_iterator::IntoEnumIterator;

#[derive(Copy, Clone, Debug, IntoEnumIterator, PartialEq)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}
#[derive(Copy, Clone, Debug, IntoEnumIterator, PartialEq)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

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
        match self.rank {
            Rank::Ace => 11,
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
        }
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