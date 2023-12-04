use std::collections::HashSet;

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

    let sum: i64 = cards
        .iter()
        .map(
            |Card {
                 id: _,
                 winning_numbers,
                 numbers_you_have,
             }| {
                let got_numbers: HashSet<_> =
                    winning_numbers.intersection(numbers_you_have).collect();

                if got_numbers.is_empty() {
                    0
                } else {
                    2i64.pow((got_numbers.len() - 1) as u32)
                }
            },
        )
        .sum();

    sum
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
    use rstest::rstest;

    #[test]
    fn it_works() {
        let input = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(part(input), 13);
    }

    #[rstest]
    #[case("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", 8)]
    #[case("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19", 2)]
    #[case("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1", 2)]
    #[case("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83", 1)]
    #[case("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36", 0)]
    #[case("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", 0)]
    fn card_by_card(#[case] card: &str, #[case] result: i64) {
        assert_eq!(part(card), result);
    }
}
