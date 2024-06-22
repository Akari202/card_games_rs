use std::error::Error;
use std::{fmt, io};
use playin_cards::Rank::{*, self};
use playin_cards::Card;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator, IndexedParallelIterator};
use crate::deck::Deck;
use colored::*;

const WILD_CARD: Rank = Queen;
const STARTING_HAND_SIZE: u8 = 10;
const PLAYERS: usize = 2;

struct TrashGame {
    player_hand_sizes: Vec<u8>,
    round: TrashRound
}

struct TrashRound {
    deck: Deck,
    hands: Vec<TrashHand>,
    discard: Option<Card>,
    turn: usize,
    player_count: usize
}

struct TrashHand {
    hand: Vec<(Card, bool)>
}

impl fmt::Display for TrashHand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in &self.hand {
            if i.1 {
                write!(f, "{} ", i.0)?;
            } else {
                write!(f, "🂠 ")?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for TrashRound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Turn number {}", self.turn)?;
        if self.discard.is_some() {
            writeln!(f, "Deck: 🂠\nDiscard: {}", self.discard.unwrap())?;
        } else {
            writeln!(f, "Deck: 🂠")?;
        }
        self.hands.iter().enumerate().for_each(|(i, j)| {
            if i == self.turn % self.player_count {
                writeln!(f, "{} {}:", "Player".bold(), i.to_string().bold()).unwrap();
            } else {    
                writeln!(f, "Player {}:", i).unwrap();
            }
            writeln!(f, "{}", j).unwrap();
        });
        Ok(())
    }
}

impl TrashGame {
    fn new() -> Self {
        let player_hand_sizes: Vec<u8> = vec![STARTING_HAND_SIZE; PLAYERS];
        Self { 
            player_hand_sizes: player_hand_sizes.clone(), 
            round: TrashRound::new(&player_hand_sizes)
        }
    }

    fn get_winner(&self) -> Option<usize> {
        let winner  = self
            .player_hand_sizes
            .par_iter()
            .enumerate()
            .min_by_key(|(_, j)| *j);
        if winner.is_some() && winner.unwrap().1 == &0 {
            Some(winner.unwrap().0)
        } else {
            None
        }
    }

    fn play_game(&mut self) {
        while self.get_winner().is_none() {
            let round_winner: usize = self.round.play_round();
            self.player_hand_sizes[round_winner] -= 1;
            self.round = TrashRound::new(&self.player_hand_sizes);
        }
        let winner: usize = self.get_winner().unwrap();
        println!("Player {} is the winner", winner);
    }
}

impl TrashRound {
    fn new(player_hand_sizes: &Vec<u8>) -> Self {
        let mut deck: Deck = Deck::new_shuffled();
        let hands: Vec<TrashHand> = player_hand_sizes
            .iter()
            .map(|i| {
                TrashHand::new(&deck.draw_cards(*i as usize))
            })
            .collect();
        let player_count: usize = hands.len();
        Self {
            deck,
            hands,
            discard: None,
            // TODO: make the turn rotate properly
            turn: 0,
            player_count
        }
    }

    fn get_winner(&self) -> Option<usize> {
        // WARN: Why is this causing issues????
        // WHY??????????
        // for (i, j) in self.hands.iter().enumerate() {
        //     if j.has_won() {
        //         return Some(i);
        //     }
        // }
        None
    }

    fn take_discard_card(&mut self) -> bool {
        false
    }

    fn take_deck_card(&mut self) -> bool {

        if self.deck.get_deck_length() >= 1 {
            let card: Card = self.deck.draw_one_card();
            println!("\nYou drew a {}", card);
            self.take_card(card);
            true
        } else {
            println!("No more cards in the deck");
            false
        }
    }

    fn take_card(&mut self, card: Card) {
        let player_turn: usize = self.turn % self.player_count;
        // WARN: I hate everything i have done here. if this is correct i hate it but it probably isnt
        let card_rank: usize;
        match card {
            Card::Regular {rank, suit: _} => {
                card_rank = rank as usize;
            },
            _ => panic!()
        }

        // println!("Player {}'s hand: {:?} with the card {} and length {}", player_turn, self.hands[player_turn].hand, card, self.hands[player_turn].hand.len());

        if self.hands[player_turn].hand.len() < card_rank {
            self.discard = Some(card);
            // println!("{} should be {}", self.discard.unwrap(), card);
            // println!("{}", self);
        } else {
            let flipped_card: (Card, bool) = self.hands[player_turn].hand[card_rank - 1];
            if flipped_card.1 {
                match flipped_card.0 {
                    Card::Regular {rank: Queen, suit: _} => {
                        self.discard = Some(card)
                    },
                    Card::Regular {rank: _, suit: _} => {self.discard = Some(card)},
                    _ => panic!()
                }
            } else {
                self.hands[player_turn].hand[card_rank - 1] = (card, true);
                self.take_card(flipped_card.0);
            }
        }
    }

    fn take_turn(&mut self) {
        // NOTE: this might be better to just be included in the struct and rolled over when incrementing the turn
        // let player_turn: usize = self.turn % self.player_count;
        println!("\n{}", self);
        loop {
            println!(
                "Do you want to\n[1]: take a {} from the discard\n[2]: draw an unknown 🂠 from the deck?",
                self.discard.unwrap_or(Card::Joker(playin_cards::Color::Black))
            );
            let mut stdin_read_buffer: String = String::new();
            io::stdin().read_line(&mut stdin_read_buffer).unwrap();
            if stdin_read_buffer.len() == 2 {
                let choice: u8 = stdin_read_buffer.trim().parse::<u8>().unwrap();
                match choice {
                    1 => { if self.take_discard_card() { break; } },
                    2 => { if self.take_deck_card() { break; } },
                    _ => ()
                }
            }
        }
    }

    fn play_round(&mut self) -> usize {
        while self.get_winner().is_none() {
            self.take_turn();
            self.turn += 1;
        }
        self.get_winner().unwrap()
    }
}

impl TrashHand {
    fn new(deck: &Vec<Card>) -> Self {
        let hand: Vec<(Card, bool)> = deck.par_iter().map(|i| (*i, false)).collect();
        Self {
            hand
        }
    }

    fn has_won(&self) -> bool {
        *self.hand.par_iter().map(|(_, i)| i).max().unwrap()
    }
}

pub fn trash() -> Result<(), Box<dyn Error>> {
    let mut game: TrashGame = TrashGame::new();
    game.play_game();
    Ok(())
}