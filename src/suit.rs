use std::fmt;

use enum_iterator::IntoEnumIterator;

#[derive(Copy, Clone, Debug, IntoEnumIterator, PartialEq, Eq, Ord, PartialOrd)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl Suit {
    pub fn get_suit_by_string(suit: &str) -> Option<Suit> {
        match suit {
            "C" => Some(Suit::Clubs),
            "D" => Some(Suit::Diamonds),
            "H" => Some(Suit::Hearts),
            "S" => Some(Suit::Spades),
            _ => None,
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
        };
        fmt::Display::fmt(suit, f)
    }
}