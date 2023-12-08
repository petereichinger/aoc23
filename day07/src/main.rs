mod part01;
mod part02;

mod game_part_1;

pub const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Part 1: {}", part01::part(INPUT));
    println!("Part 2: {}", part02::part(INPUT));
}
