use std::{collections::HashSet, str::FromStr};

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Move {
    direction: Direction,
    amount: u32,
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn move_in_direction(&self, direction: &Direction) -> Position {
        match direction {
            Direction::Up => Position {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Down => Position {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Left => Position {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right => Position {
                x: self.x + 1,
                y: self.y,
            },
        }
    }

    fn is_adjacent(&self, other: &Position) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }

    fn follow(&self, head: &Position) -> Position {
        let dx = head.x - self.x;
        let dy = head.y - self.y;

        Position {
            x: self.x + dx.signum(),
            y: self.y + dy.signum(),
        }
    }
}

fn parse_input(input: &str) -> Vec<Move> {
    input
        .lines()
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            let direction_str = parts.next()?;
            let amount = parts.next()?.parse::<u32>().ok()?;
            let direction = match direction_str {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => return None,
            };
            Some(Move { direction, amount })
        })
        .collect()
}

fn simulate_moves(moves: &[Move]) -> usize {
    let mut head = Position { x: 0, y: 0 };
    let mut tail = Position { x: 0, y: 0 };
    let mut visited_positions: HashSet<Position> = HashSet::new();
    visited_positions.insert(tail);

    for m in moves {
        for _ in 0..m.amount {
            head = head.move_in_direction(&m.direction);
            if !tail.is_adjacent(&head) {
                tail = tail.follow(&head);
                visited_positions.insert(tail);
            }
        }
    }

    visited_positions.len()
}

fn part1(input: &str) -> String {
    let moves = parse_input(input);
    let unique_positions = simulate_moves(&moves);
    unique_positions.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1(
            "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2",
        );
        assert_eq!(result, "13");
    }
}
