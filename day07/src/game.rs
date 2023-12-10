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

    pub fn new_jokers(game_str: &str) -> Self {
        const CARD_ORDER: &str = "AKQT98765432J";
        let (cards, bet) = game_str.split_once(' ').unwrap();
        let bet = bet.parse::<i64>().unwrap();
        let mut cards_map = HashMap::new();

        cards.chars().for_each(|c| {
            *cards_map.entry(c).or_insert(0u8) += 1;
        });

        let num_jokers = *cards_map.get(&'J').unwrap_or(&0);

        cards_map.remove(&'J');

        let card_counts = cards_map
            .iter()
            .sorted_by(|a, b| a.1.cmp(b.1).reverse())
            .map(|g| *g.1)
            .collect_vec();

        let game_type = game_type_with_jokers(&card_counts, num_jokers);

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

const FIVE_OAK: u8 = 7;
const FOUR_OAK: u8 = 6;
const FULL_HOUSE: u8 = 5;
const THREE_OAK: u8 = 4;
const TWO_PAIRS: u8 = 3;
const ONE_PAIR: u8 = 2;
const HIGH_CARD: u8 = 1;

fn game_type_from_ordered_card_counts(card_counts: &Vec<u8>) -> u8 {
    let len = card_counts.len();
    if len == 1 {
        FIVE_OAK
    } else if len == 2 {
        if card_counts[1] == 1 {
            FOUR_OAK
        } else {
            FULL_HOUSE
        }
    } else if len == 3 {
        if card_counts[1] == 1 && card_counts[2] == 1 {
            THREE_OAK
        } else {
            TWO_PAIRS
        }
    } else if len == 4 {
        ONE_PAIR
    } else {
        HIGH_CARD
    }
}

fn game_type_with_jokers(card_counts: &Vec<u8>, joker_count: u8) -> u8 {
    let len = card_counts.len();

    if joker_count == 0 {
        game_type_from_ordered_card_counts(card_counts)
    } else if joker_count == 1 {
        match len {
            1 => FIVE_OAK,
            2 => match card_counts[0] {
                3 => FOUR_OAK,
                2 => FULL_HOUSE,
                _ => panic!(),
            },
            3 => THREE_OAK,
            4 => ONE_PAIR,
            _ => panic!(),
        }
    } else if joker_count == 2 {
        match len {
            1 => FIVE_OAK,
            2 => FOUR_OAK,
            3 => THREE_OAK,
            _ => panic!(),
        }
    } else if joker_count == 3 {
        match len {
            1 => FIVE_OAK,
            2 => FOUR_OAK,
            _ => panic!(),
        }
    } else {
        FIVE_OAK
    }
}
