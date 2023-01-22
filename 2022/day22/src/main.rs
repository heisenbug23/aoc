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

#[derive(Clone, Debug)]
struct Position {
    row: usize,
    col: usize,
    facing: u32,
}

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

fn step(grid: &Vec<Vec<u8>>, pos: &Position, movement: &Movement) -> Position {
    match movement {
        Forward(mut steps) => {
            let mut last_pos = pos.clone();

            while steps > 0 {
                let mut new_pos = last_pos.clone();

                loop {
                    if new_pos.facing == 0 {
                        new_pos.col = (new_pos.col + 1) % grid[new_pos.row].len();
                    } else if new_pos.facing == 90 {
                        new_pos.row = (new_pos.row + 1) % grid.len();
                    } else if new_pos.facing == 180 {
                        new_pos.col = if new_pos.col == 0 { grid[new_pos.row].len() } else { new_pos.col } - 1;
                    } else {
                        new_pos.row = if new_pos.row == 0 { grid.len() } else { new_pos.row } - 1;
                    }

                    if grid[new_pos.row][new_pos.col] != b' ' {
                        break;
                    }
                }

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

fn part1(rows: &Vec<String>) -> usize {
    let (grid, movements) = parse(rows);

    let mut pos = Position {
        row: 0,
        col: grid[0].iter().enumerate().find(|(_, &c)| c == b'.').unwrap().0,
        facing: 0,
    };

    for movement in &movements {
        pos = step(&grid, &pos, movement);
    }

    1000 * (pos.row + 1) + 4 * (pos.col + 1) + pos.facing as usize / 90
}

fn part2(_: &Vec<String>) -> usize {
    todo!();
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let filename = if args.len() == 2 { &args[1] } else { "input" };
    let lines = readlines(filename);
    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let lines = readlines("test.in");
        assert_eq!(part1(&lines), 6032);
        // assert_eq!(part2(&lines), 5031);
    }
}
