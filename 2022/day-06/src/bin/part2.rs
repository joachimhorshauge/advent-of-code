use std::collections::HashSet;

fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let mut res = 0;
    for i in 14..input.len() {
        let marker = input[i - 14..i].to_string();
        let set: HashSet<_> = marker.chars().collect();
        if set.len() == 14 {
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
    fn test_part2_1() {
        let result = part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        assert_eq!(result, "19");
    }

    #[test]
    fn test_part2_2() {
        let result = part2("bvwbjplbgvbhsrlpgdmjqwftvncz");
        assert_eq!(result, "23");
    }

    #[test]
    fn test_part2_3() {
        let result = part2("nppdvjthqldpwncqszvftbrmjlhg");
        assert_eq!(result, "23");
    }

    #[test]
    fn test_part2_4() {
        let result = part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        assert_eq!(result, "29");
    }

    #[test]
    fn test_part2_5() {
        let result = part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        assert_eq!(result, "26");
    }
}
