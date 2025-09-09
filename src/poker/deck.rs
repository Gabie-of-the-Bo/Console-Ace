use rand::{rng, seq::SliceRandom};

use crate::poker::card::{Card, Suit};

pub struct Deck {
    pub cards: Vec<Card>
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = vec!();

        for suit in [Suit::Hearts, Suit::Clubs, Suit::Diamonds, Suit::Spades] {
            for number in 1..=13 {
                cards.push(Card::new(number, suit.clone()));
            }
        }

        Deck { cards }
    }

    pub fn reset_draw_cache(&mut self) {
        self.cards.iter_mut().for_each(Card::reset_draw_cache);
    }

    pub fn shuffle(&mut self) {
        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn pop(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn push(&mut self, card: Card) {
        self.cards.push(card);
    }
}