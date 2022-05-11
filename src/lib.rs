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
    None,
}
#[derive(Copy, Clone, Debug, IntoEnumIterator, PartialEq)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
    None,
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
        Rank::get_value(&self.rank)
    }
}

impl Rank {
    pub fn get_rank_by_string(rank: &str) -> Rank {
        match rank {
            "2" => Rank::Two,
            "3" => Rank::Three,
            "4" => Rank::Four,
            "5" => Rank::Five,
            "6" => Rank::Six,
            "7" => Rank::Seven,
            "8" => Rank::Eight,
            "9" => Rank::Nine,
            "10" => Rank::Ten,
            "J" => Rank::Jack,
            "Q" => Rank::Queen,
            "K" => Rank::King,
            "A" => Rank::Ace,
            _ => Rank::None,
        }
    }

    pub fn get_value(&self) -> u8 {
        match self.rank {
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
            Rank::Ace => 11, //To use as 1, %10 everything
            Rank::None => 0,
        }
    }
}

impl Suit {
    pub fn get_suit_by_string(suit: &str) -> Suit {
        match suit {
            "C" => Suit::Clubs,
            "D" => Suit::Diamonds,
            "H" => Suit::Hearts,
            "S" => Suit::Spades,
            _ => Suit::None,
        }
    }
}

pub fn deck_52() -> Vec<Card> {
    let mut deck = Vec::new();
    for rank in Rank::into_enum_iter() {
        for suit in Suit::into_enum_iter() {
            if rank == Rank::None {
                continue;
            }
            if suit == Suit::None {
                continue;
            }
            deck.push(Card::new(rank, suit));
        }
    }
    deck
}

pub fn deck_32() -> Vec<Card> {
    let mut deck = Vec::new();
    for rank in Rank::into_enum_iter().skip(5) {
        for suit in Suit::into_enum_iter() {
            if rank == Rank::None {
                continue;
            }
            if suit == Suit::None {
                continue;
            }
            deck.push(Card::new(rank, suit));
        }
    }
    deck
}

pub fn hello_world() {
    println!("Hello from Rust Cards!!");
}