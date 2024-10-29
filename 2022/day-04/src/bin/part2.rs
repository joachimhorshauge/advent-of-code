fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let mut sum = 0;

    for line in input.lines() {
        let pairs: Vec<(i32, i32)> = line
            .split(',')
            .map(|pair| {
                let mut bounds = pair.split('-');
                let start = bounds
                    .next()
                    .expect("Invalid Input")
                    .parse::<i32>()
                    .expect("Not a number");
                let end = bounds
                    .next()
                    .expect("Invalid Input")
                    .parse::<i32>()
                    .expect("Not a number");
                (start, end)
            })
            .collect();

        if pairs[0].0 <= pairs[1].1 && pairs[0].1 >= pairs[1].0 {
            sum += 1;
        }
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let result = part2(
            "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
",
        );
        assert_eq!(result, "4");
    }
}
