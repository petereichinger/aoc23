use std::collections::HashMap;

use itertools::Itertools;

use super::common::*;

pub fn part(input: &str) -> i64 {
    let trim_chars: &[_] = &['(', ')'];
    let (instructions, map_str) = input.split_once("\n\n").unwrap();

    let map: HashMap<_, _> = map_str
        .lines()
        .map(|l| {
            let (src, dsts) = l.split_once(" = ").unwrap();

            let (left, right) = dsts.trim_matches(trim_chars).split_once(", ").unwrap();

            (src, (left, right))
        })
        .collect();

    let paths: Vec<_> = map.keys().copied().filter(|k| k.ends_with('A')).collect();

    let lengths = paths
        .iter()
        .copied()
        .map(|path| {
            let mut current = path;
            let instructions = instructions.chars().into_iter().cycle();
            let steps = instructions.enumerate().take_while(|&(_, instruction)| {
                match instruction {
                    'L' => current = map[current].0,
                    'R' => current = map[current].1,
                    _ => panic!(),
                }

                !current.ends_with('Z')
            });

            (steps.last().unwrap().0 + 2) as i64
        })
        .collect_vec();

    lcm(&lengths)
}

fn lcm(nums: &[i64]) -> i64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(part(input), 6)
    }
}
