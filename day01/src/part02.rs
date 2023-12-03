pub fn part02(input: &str) -> i64 {
    let numbers: i64 = input
        .lines()
        .into_iter()
        .map(|l| replace_line_numbers(l))
        .inspect(|l| println!("{}", l))
        .map(|l| {
            (
                l.chars().find(|c| c.is_numeric()),
                l.chars().rfind(|c| c.is_numeric()),
            )
        })
        .filter_map(|l| match l {
            (Some(f), Some(s)) => Some(format!("{}{}", f, s)),
            _ => None,
        })
        .map(|num| num.parse::<i64>().unwrap())
        .sum();

    numbers
}

fn replace_line_numbers(mut line: &str) -> String {
    let mut result = String::new();

    while let Some((offset, len, number)) = find_first_number(line) {
        result.push_str(&line[0..offset]);
        result.push_str(number);
        line = &line[(offset + 1)..];
    }

    result.push_str(line);

    result
}

const PATTERNS: [(&str, &str); 9] = [
    ("one", "1"),
    ("two", "2"),
    ("three", "3"),
    ("four", "4"),
    ("five", "5"),
    ("six", "6"),
    ("seven", "7"),
    ("eight", "8"),
    ("nine", "9"),
];

fn find_first_number(line: &str) -> Option<(usize, usize, &str)> {
    PATTERNS
        .iter()
        .filter_map(|&(pattern, value)| line.find(pattern).map(|m| (m, pattern.len(), value)))
        .min_by(|f, s| f.0.cmp(&s.0))
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn it_works() {
        let test_input = r"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        assert_eq!(part02(test_input), 281);
    }
}
