use std::collections::HashSet;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut res = 0;
    for i in 4..input.len() {
        let marker = input[i - 4..i].to_string();
        let set: HashSet<_> = marker.chars().collect();
        if set.len() == 4 {
            res = i;
            break;
        }
    }
    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_1() {
        let result = part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        assert_eq!(result, "7");
    }

    #[test]
    fn test_part1_2() {
        let result = part1("bvwbjplbgvbhsrlpgdmjqwftvncz");
        assert_eq!(result, "5");
    }

    #[test]
    fn test_part1_3() {
        let result = part1("nppdvjthqldpwncqszvftbrmjlhg");
        assert_eq!(result, "6");
    }

    #[test]
    fn test_part1_4() {
        let result = part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        assert_eq!(result, "10");
    }

    #[test]
    fn test_part1_5() {
        let result = part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        assert_eq!(result, "11");
    }
}
