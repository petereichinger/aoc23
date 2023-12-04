use std::collections::{HashMap, HashSet};

pub fn part(input: &str) -> i64 {
    let cards: Vec<_> = input
        .lines()
        .map(|line| {
            let (card, numbers) = line.split_once(':').unwrap();
            let id = card
                .split_ascii_whitespace()
                .nth(1)
                .unwrap()
                .parse::<u32>()
                .unwrap();

            let (winning_numbers, numbers_you_have) = numbers.split_once('|').unwrap();

            let winning_numbers = numbers_to_hash_set(winning_numbers);
            let numbers_you_have = numbers_to_hash_set(numbers_you_have);

            Card {
                id,
                winning_numbers,
                numbers_you_have,
            }
        })
        .collect();

    let max_card = cards.iter().map(|c| c.id).max().unwrap() + 1;

    let mut card_count: HashMap<u32, usize> = HashMap::new();

    cards.iter().for_each(|card| {
        let own_copies = *card_count.entry(card.id).or_insert(1);
        let matching_numbers = card
            .numbers_you_have
            .intersection(&card.winning_numbers)
            .count() as u32;

        let end = (card.id + 1 + matching_numbers).min(max_card);
        ((card.id + 1)..(end)).for_each(|c| {
            *card_count.entry(c).or_insert(1) += own_copies;
        });
    });

    card_count.iter().map(|entry| *entry.1 as i64).sum::<i64>()
}

fn numbers_to_hash_set(numbers: &str) -> HashSet<u32> {
    numbers
        .split_ascii_whitespace()
        .map(|num| num.parse::<u32>().unwrap())
        .collect()
}

struct Card {
    id: u32,
    winning_numbers: HashSet<u32>,
    numbers_you_have: HashSet<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(part(input), 30);
    }
}
