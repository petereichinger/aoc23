use itertools::Itertools;

use super::game_part_1::*;

pub fn part(input: &str) -> i64 {
    let games = input.lines().map(|l| Game::new(l)).collect_vec();

    games
        .iter()
        .sorted()
        .enumerate()
        .map(|(idx, game)| ((idx + 1) as i64) * game.bet)
        .sum::<i64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    #[test]
    fn it_works() {
        assert_eq!(part(INPUT), 6440)
    }
}
