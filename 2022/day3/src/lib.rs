use std::collections::HashMap;

pub fn part1(input: &str) -> usize{
    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut sum = 0;
    
    let halves: HashMap<&str,&str> = input
    .split("\r\n")
    .map(|full_str| {
        full_str.split_at(full_str.len()/2)
    }).collect();

    for (left, right) in &halves {
        for i in 0..left.len() {
            let item = right.chars().nth(i).unwrap();
            if left.contains(item) {
                sum += &alphabet.chars().position(|c| c == item).unwrap()+1;
                break;
            } 
        }
    }
    sum
}

pub fn part2(input: &str) -> usize{
    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut sum = 0;
    
    let groups: Vec<&str> = input
    .split("\r\n").collect();

    for chunk in groups.chunks(3) {
        let first = &chunk[0];
        for letter in first.chars() {
            if chunk[1].contains(letter) && chunk[2].contains(letter) {
                sum += &alphabet.chars().position(|c| c == letter).unwrap()+1;
                break;
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn part1_test() {
        let input = fs::read_to_string("./test_input.txt").unwrap();
        assert_eq!(157,part1(&input));
    }

    #[test]
    fn part2_test() {
        let input = fs::read_to_string("./test_input.txt").unwrap();
        assert_eq!(70,part2(&input));
    }
}