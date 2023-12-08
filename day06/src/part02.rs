use itertools::Itertools;
use rayon::prelude::*;

pub fn part(input: &str) -> i64 {
    let (times, farthest) = input.split_once("\n").unwrap();

    let times = times.trim_start_matches("Time:");
    let farthest = farthest.trim_start_matches("Distance:");

    let time: i64 = times
        .split_ascii_whitespace()
        .join("")
        .parse::<_>()
        .unwrap();
    let farthest: i64 = farthest
        .split_ascii_whitespace()
        .join("")
        .parse::<_>()
        .unwrap();

    race(time, farthest)
}

fn race(time: i64, farthest: i64) -> i64 {
    // (7 - 1) * 1 = 6 * 1
    // (7 - 2) * 2 = 5 * 2
    // (7 - 3) * 3 = 4 * 3
    // (7 - 4) * 4 = 3 * 4
    // (7 - 5) * 5 = 2 * 5
    // (7 - 6) * 6 = 1 * 6
    // time - n

    let max_time = time - 1;

    let farther = (1..=(max_time / 2))
        .into_par_iter()
        .map(|t| (time - t) * t)
        .filter(|&t| t > farthest)
        .count() as i64;

    let odd_extra = if max_time % 2 == 1 { 1 } else { 0 };

    farther * 2 + odd_extra
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn it_works() {
        assert_eq!(part(TEST), 71503)
    }
}
