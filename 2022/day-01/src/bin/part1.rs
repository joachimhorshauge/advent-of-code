fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let mut sum = 0;

    for part in &parts {
        let nums = part.split("\n");

        let mut temp = 0;
        for num in nums {
            let n: u32 = num.parse().expect("Not a valid number");
            temp += n;
        }
        if temp > sum {
            sum = temp;
        }
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1(
            "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000",
        );
        assert_eq!(result, "24000");
    }
}
