use std::fs;
use day2::part1;
use day2::part2;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    println!("{}",part1(&input));
    println!("{}",part2(&input));
}
