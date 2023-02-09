use std::fs;
use day3::part1;
use day3::part2;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    println!("{:?}",part1(&input));
    println!("{:?}",part2(&input));
}
