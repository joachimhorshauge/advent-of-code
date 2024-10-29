use std::collections::{btree_map::Range, BinaryHeap};

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let mut sum = 0;
    let mut heap = BinaryHeap::new();

    for part in &parts {
        let nums = part.split("\n");

        let mut temp = 0;
        for num in nums {
            let n: u32 = num.parse().expect("Not a valid number");
            temp += n;
        }
        heap.push(temp);
    }
    for _ in 0..3 {
        let n: u32 = heap.pop().expect("Not a valid number");
        sum += n;
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let result = part2(
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
        assert_eq!(result, "45000");
    }
}
