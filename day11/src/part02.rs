use std::collections::HashSet;

use itertools::Itertools;
use library::grid2d::Grid2D;

use glam::IVec2;

use super::common::*;

pub fn part(input: &str) -> i64 {
    let grid = Grid2D::from_input(input);

    let empty_rows: HashSet<_> = (0..grid.height())
        .filter(|y| {
            (0..grid.width()).all(|x| grid.get(x, *y).ok().is_some_and(|cell| cell.is_none()))
        })
        .map(|v| v as i32)
        .collect();

    let empty_cols: HashSet<_> = (0..grid.width())
        .filter(|x| {
            (0..grid.height()).all(|y| grid.get(*x, y).ok().is_some_and(|cell| cell.is_none()))
        })
        .map(|v| v as i32)
        .collect();

    // dbg!(&empty_rows, &empty_cols);

    let galaxies = grid
        .iter_cells_row_dominant()
        .filter_map(|(pos, cell)| match cell {
            Some('#') => Some(pos),
            _ => None,
        })
        .collect_vec();

    galaxies
        .iter()
        .copied()
        .enumerate()
        .flat_map(|(idx, a)| {
            let a_copy = a;
            let distances = galaxies
                .iter()
                .skip(idx + 1)
                .copied()
                .map(|b| {
                    let IVec2 { x: ax, y: ay } = a;
                    let IVec2 { x: bx, y: by } = b;

                    let base_distance = ((ax - bx).abs() + (ay - by).abs()) as i64;

                    let x_range = ax.min(bx)..ax.max(bx);
                    let y_range = ay.min(by)..ay.max(by);

                    let extra_cols = empty_cols
                        .iter()
                        .filter(|ecol| x_range.contains(ecol))
                        .count() as i64;

                    let extra_rows = empty_rows
                        .iter()
                        .filter(|erow| y_range.contains(erow))
                        .count() as i64;

                    base_distance + 999_999 * extra_cols + 999_999 * extra_rows
                })
                .collect_vec();

            distances
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!(part(input), 374)
    }
}
