mod part01;
mod part02;

pub const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Part 1: {}", part01::part01(INPUT));
    println!("Part 2: {}", part02::part02(INPUT));
}
