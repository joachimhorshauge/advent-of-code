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
        let outcome = match (a, b) {
            ("A", "X") => 3,
            ("A", "Y") => 6,
            ("A", "Z") => 0,
            ("B", "X") => 0,
            ("B", "Y") => 3,
            ("B", "Z") => 6,
            ("C", "X") => 6,
            ("C", "Y") => 0,
            ("C", "Z") => 3,
            _ => panic!("Invalid input"),
        };
        let points = match b {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
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
        assert_eq!(result, "15");
    }
}
