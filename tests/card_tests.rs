#[cfg(test)]
mod tests {
    #[test]
    fn get_rust_cards_len_32() {
        let deck_32 = rust_cards::deck_32();
        
        println!("Deck 32 len: {}", deck_32.len());
        assert_eq!(deck_32.len(), 32);
    }

    #[test]
    fn get_rust_cards_len_52() {
        let deck_52 = rust_cards::deck_52();
        
        println!("Deck 52 len: {}", deck_52.len());
        assert_eq!(deck_52.len(), 52);
    }

    #[test]
    fn test_get_card_rank_by_string() {
        let rank1 :rust_cards::Rank = rust_cards::Rank::get_rank_by_string("2");
        let rank2 :rust_cards::Rank = rust_cards::Rank::get_rank_by_string("J");
        let rank3 :rust_cards::Rank = rust_cards::Rank::get_rank_by_string("A");

        assert_eq!(rank1, rust_cards::Rank::Two);            
        assert_eq!(rank2, rust_cards::Rank::Jack);          
        assert_eq!(rank3, rust_cards::Rank::Ace)        
    }

    #[test]
    fn test_card_equal() {
        let card1 = rust_cards::Card::new(rust_cards::Rank::Two, rust_cards::Suit::Clubs);
        let card2 = rust_cards::Card::new(rust_cards::Rank::Two, rust_cards::Suit::Clubs);

        assert_eq!(card1, card2);
    }
}
