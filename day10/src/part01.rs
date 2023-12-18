use glam::{IVec2, UVec2};
use library::grid2d::Grid2D;

use super::common::*;

pub fn part(input: &str) -> i64 {
    let width = input.lines().nth(0).and_then(|l| Some(l.len())).unwrap() as u32;
    let height = input.lines().count() as u32;

    let mut grid = Grid2D::new_empty(width, height);

    let mut start = IVec2::ZERO;

    input.lines().enumerate().for_each(|(y, row)| {
        row.chars().enumerate().for_each(|(x, content)| {
            let coord = IVec2::new(x as i32, y as i32);
            if content != '.' {
                *grid.get_mut_vec(coord).unwrap() = Some(content);
            }
            if content == 'S' {
                start = coord;
            }
        })
    });

    let mut direction = IVec2::ZERO;

    let north = start + IVec2::NEG_Y;
    let east = start + IVec2::X;
    let south = start + IVec2::Y;
    let west = start + IVec2::NEG_X;
    if let Ok(Some(cell)) = grid.get_vec(north) {
        if cell == &'|' || cell == &'F' || cell == &'7' {
            direction = IVec2::NEG_Y;
        }
    } else if let Ok(Some(cell)) = grid.get_vec(east) {
        if cell == &'-' || cell == &'7' || cell == &'J' {
            direction = IVec2::X;
        }
    } else if let Ok(Some(cell)) = grid.get_vec(south) {
        if cell == &'|' || cell == &'L' || cell == &'J' {
            direction = IVec2::Y;
        }
    } else if let Ok(Some(cell)) = grid.get_vec(west) {
        if cell == &'-' || cell == &'F' || cell == &'L' {
            direction = IVec2::NEG_X;
        }
    } else {
        panic!()
    }

    let mut current = start + direction;
    let mut count = 1;

    while current != start {
        let cell = grid.get_vec(current).unwrap().unwrap();
        count += 1;
        match (cell, direction) {
            ('7', IVec2::X) => direction = IVec2::Y,
            ('J', IVec2::X) => direction = IVec2::NEG_Y,

            ('F', IVec2::NEG_X) => direction = IVec2::Y,
            ('L', IVec2::NEG_X) => direction = IVec2::NEG_Y,

            ('J', IVec2::Y) => direction = IVec2::NEG_X,
            ('L', IVec2::Y) => direction = IVec2::X,

            ('F', IVec2::NEG_Y) => direction = IVec2::X,
            ('7', IVec2::NEG_Y) => direction = IVec2::NEG_X,

            ('-', _) => {}
            ('|', _) => {}
            _ => panic!(),
        }
        current = current + direction;
    }

    // dbg!(count);
    // todo!()
    count / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";
        assert_eq!(part(input), 4)
    }

    #[test]
    fn it_works_2() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...
";
        assert_eq!(part(input), 8)
    }
}
