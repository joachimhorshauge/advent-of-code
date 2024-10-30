fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);
    dbg!(output);
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
    let mut lines = input.lines();
    let mut stacks = Vec::new();

    while let Some(line) = lines.next() {
        if line.trim().is_empty() {
            break;
        }
        if line.trim().is_empty()
            || line
                .trim()
                .chars()
                .all(|c| c.is_digit(10) || c.is_whitespace())
        {
            continue;
        }

        let chars: Vec<_> = line.chars().collect();

        if stacks.is_empty() {
            stacks = vec![vec![]; (chars.len() + 3) / 4];
        }

        for (i, stack) in stacks.iter_mut().enumerate() {
            if 1 + i * 4 < chars.len() && chars[1 + i * 4] != ' ' {
                stack.push(chars[1 + i * 4]);
            }
        }
    }

    for stack in &mut stacks {
        stack.reverse();
    }

    let mut operations = vec![];
    for line in lines {
        if line.starts_with("move") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if let (Some(count), Some(from), Some(to)) = (
                parts[1].parse::<usize>().ok(),
                parts[3].parse::<usize>().ok(),
                parts[5].parse::<usize>().ok(),
            ) {
                operations.push((count, from - 1, to - 1));
            }
        }
    }

    (stacks, operations)
}

fn mv(stacks: &mut Vec<Vec<char>>, operation: (usize, usize, usize)) {
    let amount = operation.0;
    let from = operation.1;
    let to = operation.2;

    if stacks[from].len() >= amount {
        let len = stacks[from].len();
        let items: Vec<_> = stacks[from].drain((len - amount)..).collect();
        stacks[to].extend(items);
    }
}

fn part2(input: &str) -> String {
    let temp = parse_input(input);
    let mut stacks = temp.0;
    let operations = temp.1;

    for operation in operations {
        mv(&mut stacks, operation);
    }

    let mut result = String::new();

    for stack in stacks.iter_mut() {
        if let Some(ch) = stack.pop() {
            result.push(ch);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let result = part2(
            "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
",
        );
        assert_eq!(result, "MCD");
    }
}
