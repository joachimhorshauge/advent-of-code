use core::panic;
use std::isize;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    IResult,
};

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug)]
struct Operation {
    cost: isize,
    delta: isize,
}

fn parse_noop(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("noop")(input)?;
    Ok((input, Operation { cost: 1, delta: 0 }))
}
fn parse_addx(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("addx ")(input)?;
    let (input, n) = complete::i32(input)?;

    Ok((
        input,
        Operation {
            cost: 2,
            delta: n as isize,
        },
    ))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Operation>> {
    let (_, ops) = separated_list1(newline, alt((parse_noop, parse_addx)))(input)?;
    Ok((input, ops))
}

fn part1(input: &str) -> String {
    let Ok((_, operations)) = parse_input(input) else { panic!("Error parsing input");};

    let mut result: isize = 0;
    let mut sum: isize = 1;
    let mut cycles: isize = 0;
    let mut bound: isize = 20;
    let upperbound: isize = 220;

    for op in operations {
        cycles += op.cost;

        if cycles >= bound {
            result += bound * sum;
            bound += 40;
        };
        sum += op.delta;

        if bound > upperbound {
            break;
        };
    }
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1(
            "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
",
        );
        assert_eq!(result, "13140");
    }
}
