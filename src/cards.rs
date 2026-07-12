use rand::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Card {
    pub color: Option<String>,
    pub count: u8,
    pub symbol: Option<String>,
}

impl Card {
    /// The starting placeholder card, shown before the first draw.
    pub fn empty() -> Self {
        Card {
            color: None,
            count: 0,
            symbol: None,
        }
    }
}

pub fn init_cards() -> Vec<Card> {
    let colors: Vec<&str> = vec!["red", "yellow", "green", "blue", "purple", "orange"];
    let symbols: Vec<&str> = vec!["lollipop", "cone", "peppermint", "gumdrop", "fudge"];

    let mut cards: Vec<Card> = Vec::new();

    for color in colors {
        for _ in 0..4 {
            cards.push(Card {
                color: Some(color.to_string()),
                count: 1,
                symbol: None,
            });
        }
        for _ in 0..3 {
            cards.push(Card {
                color: Some(color.to_string()),
                count: 2,
                symbol: None,
            });
        }
    }
    for symbol in symbols {
        cards.push(Card {
            color: None,
            count: 1,
            symbol: Some(symbol.to_string()),
        });
    }

    shuffle_deck(&mut cards);

    cards
}

fn shuffle_deck<T>(vector: &mut [T]) {
    let mut rng = rand::rng();

    vector.shuffle(&mut rng);
}

pub fn get_card(in_cards: &[Card]) -> (Card, Vec<Card>) {
    let mut cards = in_cards.to_vec();
    if cards.is_empty() {
        cards = init_cards();
    }

    (cards.pop().unwrap(), cards)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_builds_a_deck() {
        let deck = init_cards();
        assert_eq!(deck.len(), 47);
        println!("{deck:#?}");
    }

    #[test]
    fn get_card_pops_one_and_returns_the_rest() {
        let deck = init_cards();
        let (card, rest) = get_card(&deck);
        assert_eq!(rest.len(), deck.len() - 1);
        assert_eq!(*deck.last().unwrap(), card);
    }

    #[test]
    fn get_card_reshuffles_a_fresh_deck_when_empty() {
        let (_card, rest) = get_card(&[]);
        // Drew from a fresh 47-card deck, leaving 46.
        assert_eq!(rest.len(), 46);
    }
}
