use std::fmt;

use enum_iterator::IntoEnumIterator;

#[derive(Copy, Clone, Debug, IntoEnumIterator, PartialEq, Eq)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
    None,
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

impl fmt::Display for Suit {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let suit = match self {
            Suit::Clubs => "Clubs",
            Suit::Diamonds => "Diamonds",
            Suit::Hearts => "Hearts",
            Suit::Spades => "Spades",
            Suit::None => panic!("None shouldnt be printed"),
        };
        fmt::Display::fmt(suit, f)
    }
}