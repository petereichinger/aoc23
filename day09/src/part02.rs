use itertools::Itertools;

use super::common::*;

pub fn part(input: &str) -> i64 {
    input.lines().map(extrapolate_sequence).sum()
}

fn extrapolate_sequence(sequence: &str) -> i64 {
    let values = sequence
        .split_ascii_whitespace()
        .filter_map(|v| v.parse::<i64>().ok())
        .rev()
        .collect_vec();

    let mut values = vec![values];

    while values.last().unwrap().iter().any(|v| v != &0) {
        let deltas = values
            .last()
            .unwrap()
            .iter()
            .tuple_windows()
            .map(|(a, b)| b - a)
            .collect_vec();

        values.push(deltas);
    }

    let mut delta = 0i64;
    (0..(values.len() - 1)).rev().for_each(|idx| {
        delta = delta + values[idx].last().unwrap();
    });

    delta
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("0 3 6 9 12 15", -3)]
    #[case("1 3 6 10 15 21", 0)]
    #[case("10 13 16 21 30 45", 5)]
    fn it_works(#[case] input: &str, #[case] result: i64) {
        assert_eq!(part(input), result)
    }
}
