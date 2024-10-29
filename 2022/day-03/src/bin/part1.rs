use std::collections::HashSet;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut sum = 0;

    for line in input.lines() {
        let mid = line.len() / 2;
        let rucksack1: HashSet<_> = line[..mid].chars().collect();
        let rucksack2: HashSet<_> = line[mid..].chars().collect();

        let intersection: Vec<_> = rucksack1.intersection(&rucksack2).collect();

        let c = **intersection
            .first()
            .expect("Error: The rucksacks have more or less than 1 item in common");

        sum += if c.is_ascii_lowercase() {
            c as u32 - 96
        } else {
            c as u32 - 38
        };
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1(
            "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw",
        );
        assert_eq!(result, "157");
    }
}
