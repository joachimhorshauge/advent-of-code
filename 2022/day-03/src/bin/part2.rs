use std::collections::HashSet;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let mut sum = 0;

    let lines: Vec<_> = input.lines().collect();

    for chunk in lines.chunks(3) {
        let rucksack1: HashSet<_> = chunk[0].chars().collect();
        let rucksack2: HashSet<_> = chunk[1].chars().collect();
        let rucksack3: HashSet<_> = chunk[2].chars().collect();

        let intersection: Vec<_> = rucksack1
            .intersection(&rucksack2)
            .cloned()
            .collect::<HashSet<_>>()
            .intersection(&rucksack3)
            .cloned()
            .collect();

        let c = *intersection
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
    fn test_part2() {
        let result = part2(
            "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw",
        );
        assert_eq!(result, "70");
    }
}
