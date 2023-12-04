use std::ops::RangeInclusive;

use itertools::Itertools;
use library::grid2d::{self, Grid2D};

pub fn part(input: &str) -> i64 {
    let width = input.lines().nth(0).unwrap().len() + 1;
    let height = input.lines().count() + 1;

    let mut grid = grid2d::Grid2D::<char>::new_empty(width, height);

    dbg!(width, height);
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, cell)| {
            if cell != '.' {
                *(grid.get_mut(x, y).unwrap()) = Some(cell);
            }
        })
    });

    let mut sum = 0;

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
                        let is_part_number = check_range(&grid, x_range.clone(), y_range.clone());
                        if is_part_number {
                            sum += current_number;
                        }
                        current_number = 0;
                        num_digits = 0;
                    }
                }
            }
        });
    });

    sum
}

fn check_range(
    grid: &Grid2D<char>,
    x_range: RangeInclusive<usize>,
    y_range: RangeInclusive<usize>,
) -> bool {
    x_range.cartesian_product(y_range).any(|(x, y)| {
        let cell = grid.get(x, y).unwrap_or(&Some('.')).unwrap_or('.');

        cell != '.' && !cell.is_numeric()
    })
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

        assert_eq!(part(input), 4361)
    }
    #[test]
    fn reddit_test_1() {
        let input = r#"1-......
........
......-1
........
.24.....
......*4"#;
        assert_eq!(part(input), 6)
    }
}
