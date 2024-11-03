fn parse_to_vec(input: &str) -> Vec<Vec<isize>> {
    input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).expect("Expected a digit") as isize)
                .collect()
        })
        .collect()
}

fn part2(input: &str) -> String {
    let grid = parse_to_vec(input);
    let rows = grid.len();
    let cols = grid[0].len();
    let mut max_score: usize = 0;

    for row in 0..rows {
        for col in 0..cols {
            let height = grid[row][col];

            let mut up_count = 0;
            let mut r = row;
            while r > 0 {
                r -= 1;
                up_count += 1;
                if grid[r][col] >= height {
                    break;
                }
            }

            let mut down_count = 0;
            let mut r = row;
            while r + 1 < rows {
                r += 1;
                down_count += 1;
                if grid[r][col] >= height {
                    break;
                }
            }

            let mut left_count = 0;
            let mut c = col;
            while c > 0 {
                c -= 1;
                left_count += 1;
                if grid[row][c] >= height {
                    break;
                }
            }

            let mut right_count = 0;
            let mut c = col;
            while c + 1 < cols {
                c += 1;
                right_count += 1;
                if grid[row][c] >= height {
                    break;
                }
            }

            let scenic_score = up_count * down_count * left_count * right_count;
            if scenic_score > max_score {
                max_score = scenic_score;
            }
        }
    }

    max_score.to_string()
}

fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);
    dbg!(output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let result = part2(
            "30373
25512
65332
33549
35390",
        );
        assert_eq!(result, "8");
    }
}
