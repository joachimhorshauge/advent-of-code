fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut score = 0;
    for line in input.lines() {
        let mut iter = line.split_whitespace();
        let a = iter.next().expect("Invalid input");
        let b = iter.next().expect("Invalid input");
        let points = match (a, b) {
            ("A", "X") => 3,
            ("A", "Y") => 1,
            ("A", "Z") => 2,
            ("B", "X") => 1,
            ("B", "Y") => 2,
            ("B", "Z") => 3,
            ("C", "X") => 2,
            ("C", "Y") => 3,
            ("C", "Z") => 1,
            _ => panic!("Invalid input"),
        };
        let outcome = match b {
            "X" => 0,
            "Y" => 3,
            "Z" => 6,
            _ => panic!("Invalid input"),
        };
        score += outcome + points;
    }
    score.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1(
            "A Y
B X
C Z",
        );
        assert_eq!(result, "12");
    }
}
