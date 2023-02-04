use std::fs;
use day1::process_part1;
use day1::process_part2;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    println!("{}",process_part1(&input));
    println!("{}",process_part2(&input));
}

