use std::fmt;

use playin_cards::{Card, gen_shoe};
use rand::{thread_rng, seq::SliceRandom};

pub struct Deck {
    deck: Vec<Card>
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in &self.deck {
            write!(f, "{} ", i)?;
        }
        Ok(())
    }
}

impl Deck {
    pub fn new_unshuffled() -> Self {
        Self {
            deck: gen_shoe(1, false)
        }
    }

    pub fn new_shuffled() -> Self {
        let mut deck: Vec<Card> = gen_shoe(1, false);
        let mut rng = thread_rng();
        deck.shuffle(&mut rng);
        Self {
            deck
        }
    }

    pub fn draw_cards(&mut self, count: usize) -> Vec<Card> {
        let drawn: Vec<Card> = self.deck[0..count].to_vec();
        let deck_length: usize = self.deck.len();
        self.deck = self.deck[(count + 1)..deck_length].to_vec();
        drawn
    }

    pub fn draw_one_card(&mut self) -> Card {
        let drawn: Card = self.deck[0];
        let deck_length: usize = self.deck.len();
        self.deck = self.deck[1..deck_length].to_vec();
        drawn
    }

    pub fn get_deck_length(&self) -> usize {
        self.deck.len()
    }
}