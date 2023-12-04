use std::{collections::HashMap, ops::RangeInclusive};

use itertools::Itertools;
use library::grid2d::{self, Grid2D};

pub fn part(input: &str) -> i64 {
    let width = input.lines().nth(0).unwrap().len() + 1;
    let height = input.lines().count() + 1;

    let mut grid = grid2d::Grid2D::<char>::new_empty(width, height);

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, cell)| {
            if cell != '.' {
                *(grid.get_mut(x, y).unwrap()) = Some(cell);
            }
        })
    });

    let mut gears: HashMap<(usize, usize), Vec<i64>> = HashMap::new();

    (0..height).for_each(|y| {
        let mut current_number = 0i64;
        let mut num_digits = 0usize;
        (0..width).for_each(|x| {
            let cell = grid.get(x, y).unwrap();
            match cell {
                Some(content) if content.is_numeric() => {
                    current_number = current_number * 10 + content.to_digit(10).unwrap() as i64;
                    num_digits += 1;
                }
                _ => {
                    if num_digits > 0 {
                        let x_range = x.saturating_sub(num_digits + 1)..=x;
                        let y_range = y.saturating_sub(1)..=y + 1;
                        let is_gear = check_range(&grid, x_range.clone(), y_range.clone());
                        if let Some(gear_pos) = is_gear {
                            gears.entry(gear_pos).or_default().push(current_number);
                        }
                        current_number = 0;
                        num_digits = 0;
                    }
                }
            }
        });
    });

    gears
        .iter()
        .filter(|(_pos, g)| g.len() == 2)
        .map(|(_pos, g)| g[0] * g[1])
        .sum()
}

fn check_range(
    grid: &Grid2D<char>,
    x_range: RangeInclusive<usize>,
    y_range: RangeInclusive<usize>,
) -> Option<(usize, usize)> {
    let gear: Vec<_> = x_range
        .cartesian_product(y_range)
        .filter(|(x, y)| {
            let cell = grid.get(*x, *y).unwrap_or(&Some('.')).unwrap_or('.');

            cell == '*'
        })
        .collect();

    gear.get(0).copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(part(input), 467835)
    }
    #[test]
    fn reddit_test_1() {
        let input = r#"1-......
........
......-1
........
.24.....
......*4"#;
        assert_eq!(part(input), 0)
    }
}
