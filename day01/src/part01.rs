pub fn part01(input: &str) -> i64 {
    let numbers: i64 = input
        .lines()
        .into_iter()
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
