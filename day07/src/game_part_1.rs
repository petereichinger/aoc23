use std::collections::HashMap;

use itertools::Itertools;

pub type Cards = Vec<u8>;

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq)]
pub struct Game {
    pub game_type: u8,
    pub cards: Cards,
    pub bet: i64,
}

impl Game {
    pub fn new(game_str: &str) -> Self {
        const CARD_ORDER: &str = "AKQJT98765432";

        let (cards, bet) = game_str.split_once(' ').unwrap();
        let bet = bet.parse::<i64>().unwrap();
        let mut cards_map = HashMap::new();

        cards.chars().for_each(|c| {
            *cards_map.entry(c).or_insert(0u8) += 1;
        });

        let card_counts = cards_map
            .iter()
            .sorted_by(|a, b| a.1.cmp(b.1).reverse())
            .map(|g| *g.1)
            .collect_vec();

        let game_type = game_type_from_ordered_card_counts(&card_counts);

        let cards = cards
            .chars()
            .map(|c| (CARD_ORDER.len() - CARD_ORDER.find(c).unwrap()) as u8)
            .collect();

        Self {
            game_type,
            cards,
            bet,
        }
    }
}

impl Ord for Game {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.game_type
            .cmp(&other.game_type)
            .then(self.cards.cmp(&other.cards))
    }
}

fn game_type_from_ordered_card_counts(card_counts: &Vec<u8>) -> u8 {
    if card_counts.len() == 1 {
        7 // five of a kind
    } else if card_counts.len() == 2 {
        if card_counts[1] == 1 {
            6 // four of a kind
        } else {
            5 // full house
        }
    } else if card_counts.len() == 3 {
        if card_counts[1] == 1 && card_counts[2] == 1 {
            4 // three of a kind
        } else {
            3
        }
    } else if card_counts.len() == 4 {
        2
    } else {
        1 // high card
    }
}
