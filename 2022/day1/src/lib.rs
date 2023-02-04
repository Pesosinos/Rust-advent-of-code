
pub fn process_part1(input: &str) ->String{
    let result = input
        .split("\r\n\r")
        .map(|elf_load| {
            elf_load
                .split("\r\n")
                .map(|item| {
                    item.trim().parse::<u32>().unwrap()})
                .sum::<u32>()
        })
        .max()
        .unwrap();

    result.to_string()
}

pub fn process_part2(input: &str) ->String{
    let mut result:Vec<u32> = input
        .split("\r\n\r")
        .map(|elf_load| {
            elf_load
                .split("\r\n")
                .map(|item| {
                    item.trim().parse::<u32>().unwrap()})
                .sum::<u32>()
        }).collect();

    result.sort_by(|a,b| b.cmp(a));
    let sum: u32 = result.iter().take(3).sum();
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        let result = process_part1(input);
        assert_eq!("24000",result);
    }
}