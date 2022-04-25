#[cfg(test)]
mod tests {
    #[test]
    fn get_cards_len_32() {
        let deck_32 = rust_cards::deck_32();
        
        println!("Deck 32 len: {}", deck_32.len());
        assert_eq!(deck_32.len(), 32);
    }

    #[test]
    fn get_cards_len_52() {
        let deck_52 = rust_cards::deck_52();
        
        println!("Deck 52 len: {}", deck_52.len());
        assert_eq!(deck_52.len(), 52);
    }
}
