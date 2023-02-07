pub fn part1(input: &str) -> i32{
    let pairs: Vec<&str> = input.split("\r\n").collect();
    let mut sum = 0;
    for pair in pairs{
        let right = pair.chars().nth(2).unwrap();
        let left = pair.chars().nth(0).unwrap();
       
        match right {
            'X' => {sum +=1;
                match left {
                    'C' => {sum +=6},
                    'A' => {sum +=3},
                    _ => ()
                }
            },
            'Y' => {sum +=2;
                match left {
                    'A' => {sum +=6},
                    'B' => {sum +=3},
                    _ => ()
                }
            },
            'Z' => {sum +=3;
                match left {
                    'B' => {sum +=6},
                    'C' => {sum +=3},
                    _ => ()
                }
            },
            _ => ()
        }
    }
    sum
}

pub fn part2(input: &str) -> i32{
    let pairs: Vec<&str> = input.split("\r\n").collect();
    let mut sum = 0;
    for pair in pairs{
        let right = pair.chars().nth(2).unwrap();
        let left = pair.chars().nth(0).unwrap();
       
        match right {
            'X' => {
                match left {
                    'A' => {sum +=3},
                    'B' => {sum +=1},
                    'C' => {sum +=2},
                    _ => ()
                }
            },
            'Y' => {
                match left {
                    'A' => {sum +=4},
                    'B' => {sum +=5},
                    'C' => {sum +=6},
                    _ => ()
                }
            },
            'Z' => {
                match left {
                    'A' => {sum +=8},
                    'B' => {sum +=9},
                    'C' => {sum +=7},
                    _ => ()
                }
            },
            _ => ()
        }
    }
    sum
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1_test() {
        let test_input = fs::read_to_string("./test_input.txt").unwrap();

        assert_eq!(15,part1(&test_input));
    }

    #[test]
    fn part2_test() {
        let test_input = fs::read_to_string("./test_input.txt").unwrap();

        assert_eq!(12,part2(&test_input));
    }

}