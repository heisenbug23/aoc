use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::Movement::*;

fn readlines(filename: &str) -> Vec<String> {
    let file = BufReader::new(File::open(&filename)
        .expect(format!("File \"{filename}\" not found").as_ref()));
    return file.lines().map(Result::unwrap).collect();
}

#[derive(Clone, Debug)]
enum Movement {
    Forward(u8),
    RotateClockwise,
    RotateCounterclockwise,
}

#[derive(Clone, Debug, PartialEq)]
struct Position {
    row: usize,
    col: usize,
    facing: u32, // 0 = right, then in steps of 90 clockwise
}

type Warp = fn(grid: &Vec<Vec<u8>>, &Position) -> Option<Position>;

fn parse(rows: &Vec<String>) -> (Vec<Vec<u8>>, Vec<Movement>) {
    let mut grid = vec!();
    let max_width = rows.iter().rev().skip(2).map(|r| r.len()).max().unwrap();
    for row in rows.iter().take(rows.len() - 1) {
        grid.push(Vec::from(row.as_bytes()));
        grid.last_mut().unwrap().extend(vec![b' '; max_width - row.len()])
    }

    let mut movements = vec!();
    let mut chars: Vec<u8> = vec!();
    for &c in rows.last().unwrap().as_bytes() {
        if c.is_ascii_digit() {
            chars.push(c);
        } else {
            if !chars.is_empty() {
                movements.push(Forward(String::from_utf8(chars.clone()).unwrap().parse().unwrap()));
                chars.clear();
            }
            movements.push(if c == b'R' { RotateClockwise } else { RotateCounterclockwise });
        }
    }
    if !chars.is_empty() {
        movements.push(Forward(String::from_utf8(chars.clone()).unwrap().parse().unwrap()));
    }

    (grid, movements)
}

fn facing_to_move_vector(facing: u32) -> (isize, isize) {
    if facing == 0 {
        (0, 1)
    } else if facing == 90 {
        (1, 0)
    } else if facing == 180 {
        (0, -1)
    } else {
        (-1, 0)
    }
}

fn do_movement(grid: &Vec<Vec<u8>>, pos: &Position, movement: &Movement, warp: Warp) -> Position {
    match movement {
        Forward(mut steps) => {
            let mut last_pos = pos.clone();

            while steps > 0 {
                let new_pos = match warp(grid, &last_pos) {
                    Some(p) => p,
                    None => {
                        let dir = facing_to_move_vector(last_pos.facing);
                        Position {
                            row: (last_pos.row as isize + dir.0 + grid.len() as isize) as usize % grid.len(),
                            col: (last_pos.col as isize + dir.1 + grid[last_pos.row].len() as isize) as usize % grid[last_pos.row].len(),
                            facing: last_pos.facing,
                        }
                    }
                };

                if grid[new_pos.row][new_pos.col] == b'#' {
                    break;
                }

                last_pos = new_pos;
                steps -= 1;
            }

            last_pos
        }
        RotateClockwise => Position {
            facing: (pos.facing + 90) % 360,
            ..*pos
        },
        RotateCounterclockwise => Position {
            facing: (pos.facing + 270) % 360,
            ..*pos
        }
    }
}

fn part1(rows: &Vec<String>,) -> usize {
    let (grid, movements) = parse(rows);

    let mut pos = Position {
        row: 0,
        col: grid[0].iter().enumerate().find(|(_, &c)| c == b'.').unwrap().0,
        facing: 0,
    };

    for movement in &movements {
        pos = do_movement(&grid, &pos, movement, |grid, pos| {
            let dir = facing_to_move_vector(pos.facing);

            let mut new_pos = pos.clone();
            loop {
                new_pos.row = (new_pos.row as isize + dir.0 + grid.len() as isize) as usize % grid.len();
                new_pos.col = (new_pos.col as isize + dir.1 + grid[new_pos.row].len() as isize) as usize % grid[new_pos.row].len();

                if grid[new_pos.row][new_pos.col] != b' ' {
                    break;
                }
            }

            Some(new_pos)
        });
    }

    1000 * (pos.row + 1) + 4 * (pos.col + 1) + pos.facing as usize / 90
}

fn part2(rows: &Vec<String>, warp: Warp) -> usize {
    let (grid, movements) = parse(rows);

    let mut pos = Position {
        row: 0,
        col: grid[0].iter().enumerate().find(|(_, &c)| c == b'.').unwrap().0,
        facing: 0,
    };

    for movement in &movements {
        pos = do_movement(&grid, &pos, movement, warp);
    }

    1000 * (pos.row + 1) + 4 * (pos.col + 1) + pos.facing as usize / 90
}

fn wrap_input(_: &Vec<Vec<u8>>, pos: &Position) -> Option<Position> {
    // Layout:
    //   1 3
    //   2
    // 4 6
    // 5
    match (pos.row, pos.col, pos.facing) {
        // 1 -> 4
        (0..=49, 50, 180) => Some(Position { row: 149 - pos.row, col: 0, facing: 0 }),
        // 1 -> 5
        (0, 50..=99, 270) => Some(Position { row: 100 + pos.col, col: 0, facing: 0 }),

        // 2 -> 3
        (50..=99, 99, 0) => Some(Position { row: 49, col: 50 + pos.row, facing: 270 }),
        // 2 -> 4
        (50..=99, 50, 180) => Some(Position { row: 100, col: pos.row - 50, facing: 90 }),

        // 3 -> 6
        (0..=49, 149, 0) => Some(Position { row: 149 - pos.row, col: 99, facing: 180 }),
        // 3 -> 2
        (49, 100..=149, 90) => Some(Position { row: pos.col - 50, col: 99, facing: 180 }),
        // 3 -> 5
        (0, 100..=149, 270) => Some(Position { row: 199, col: pos.col - 100, facing: 270 }),

        // 4 -> 1
        (100..=149, 0, 180) => Some(Position { row: 149 - pos.row, col: 50, facing: 0 }),
        // 4 -> 2
        (100, 0..=49, 270) => Some(Position { row: 50 + pos.col, col: 50, facing: 0 }),

        // 5 -> 6
        (150..=199, 49, 0) => Some(Position { row: 149, col: pos.row - 100, facing: 270 }),
        // 5 -> 3
        (199, 0..=49, 90) => Some(Position { row: 0, col: pos.col + 100, facing: 90 }),
        // 5 -> 1
        (150..=199, 0, 180) => Some(Position { row: 0, col: pos.row - 100, facing: 90 }),

        // 6 -> 3
        (100..=149, 99, 0) => Some(Position { row: 149 - pos.row, col: 149, facing: 180 }),
        // 6 -> 5
        (149, 50..=99, 90) => Some(Position { row: 100 + pos.col, col: 49, facing: 180 }),

        _ => None
    }
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let filename = if args.len() == 2 { &args[1] } else { "input" };
    let lines = readlines(filename);
    println!("Part 1: {}", part1(&lines));
    println!("Part 2 needs manual mapping, see wrap_input()");
    // println!("Part 2: {}", part2(&lines, wrap_input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let lines = readlines("test.in");
        assert_eq!(part1(&lines), 6032);
        assert_eq!(part2(&lines, |_, pos| {
            // layout:
            //     1
            // 5 4 2
            //     6 3

            match (pos.row, pos.col, pos.facing) {
                // 1 -> 3
                (0..=3, 11, 0) => Some(Position { row: 11 - pos.row, col: 15, facing: 180 }),
                // 1 -> 4
                (0..=3, 8, 180) => Some(Position { row: 4, col: 4 + pos.row, facing: 90 }),
                // 1 -> 5
                (0, 8..=11, 270) => Some(Position { row: 4, col: 11 - pos.col, facing: 90 }),

                // 2 -> 3
                (4..=7, 11, 0) => Some(Position { row: 8, col: 15 - pos.row % 4, facing: 90 }),

                // 3 -> 1
                (8..=11, 15, 0) => Some(Position { row: 11 - pos.row, col: 11, facing: 180 }),
                // 3 -> 5
                (11, 12..=15, 90) => Some(Position { row: 7 - pos.col % 4, col: 0, facing: 0 }),
                // 3 -> 2
                (8, 12..=15, 270) => Some(Position { row: 7 - pos.col % 4, col: 11, facing: 180 }),

                // 4 -> 6
                (7, 4..=7, 90) => Some(Position { row: 15 - pos.col, col: 8, facing: 0 }),
                // 4 -> 1
                (4, 4..=7, 270) => Some(Position { row: pos.col % 4, col: 8, facing: 0 }),

                // 5 -> 6
                (7, 0..=3, 90) => Some(Position { row: 11, col: 11 - pos.col, facing: 0 }),
                // 5 -> 3
                (4..=7, 0, 180) => Some(Position { row: 11, col: 15 - pos.col, facing: 180 }),
                // 5 -> 1
                (4, 0..=3, 270) => Some(Position { row: 0, col: 11 - pos.col, facing: 0 }),

                // 6 -> 5
                (11, 8..=11, 90) => Some(Position { row: 7, col: 11 - pos.col, facing: 270 }),
                // 6 -> 4
                (8..=11, 8, 180) => Some(Position { row: 7, col: 15 - pos.row, facing: 270 }),

                _ => None
            }
        }), 5031);
    }

    #[test]
    fn works_with_input() {
        let lines = readlines("input");
        assert_eq!(part1(&lines), 26558);
        assert_eq!(part2(&lines, wrap_input), 110400);
    }
}
