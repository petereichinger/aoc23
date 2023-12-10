use std::collections::HashMap;

use itertools::Itertools;

use super::common::*;

pub fn part(input: &str) -> i64 {
    let trim_chars: &[_] = &['(', ')'];
    let (instructions, map_str) = input.split_once("\n\n").unwrap();

    let mut map = HashMap::new();

    map_str.lines().for_each(|l| {
        let (src, dsts) = l.split_once(" = ").unwrap();

        let (left, right) = dsts.trim_matches(trim_chars).split_once(", ").unwrap();

        map.insert(src, (left, right));
    });

    let mut current = "AAA";

    let instructions = instructions.chars().into_iter().cycle();

    let steps = instructions.enumerate().take_while(|&(_, instruction)| {
        match instruction {
            'L' => current = map[current].0,
            'R' => current = map[current].1,
            _ => panic!(),
        }

        current != "ZZZ"
    });

    (steps.last().unwrap().0 + 2) as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(part(input), 2)
    }

    #[test]
    fn it_works_2() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(part(input), 6)
    }
}
