use std::{collections::HashSet, isize, usize};

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn parse_to_vec(input: &str) -> Vec<Vec<isize>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("Expected a digit") as isize)
                .collect()
        })
        .collect()
}

fn part1(input: &str) -> String {
    let mut visible_trees: HashSet<(usize, usize)> = HashSet::new();
    let grid = parse_to_vec(input);
    let rows = grid.len();
    let cols = grid[0].len();
    let mut max_height: isize;

    for row in 0..rows {
        max_height = -1;
        for col in 0..cols {
            let height = grid[row][col];
            if height > max_height {
                visible_trees.insert((row, col));
                max_height = height;
            }
        }

        max_height = -1;
        for col in (0..cols).rev() {
            let height = grid[row][col];
            if height > max_height {
                visible_trees.insert((row, col));
                max_height = height;
            }
        }
    }

    for col in 0..cols {
        max_height = -1;
        for row in 0..rows {
            let height = grid[row][col];
            if height > max_height {
                visible_trees.insert((row, col));
                max_height = height;
            }
        }

        max_height = -1;
        for row in (0..rows).rev() {
            let height = grid[row][col];
            if height > max_height {
                visible_trees.insert((row, col));
                max_height = height;
            }
        }
    }

    println!("Grid with visible trees:");
    for row in 0..rows {
        for col in 0..cols {
            if visible_trees.contains(&(row, col)) {
                print!("*");
            } else {
                print!(".");
            }
        }
        println!();
    }

    visible_trees.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1(
            "30373
25512
65332
33549
35390",
        );
        assert_eq!(result, "21");
    }
}
