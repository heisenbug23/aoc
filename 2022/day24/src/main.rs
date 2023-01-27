use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn readlines(filename: &str) -> Vec<String> {
    let file = BufReader::new(File::open(&filename)
        .expect(format!("File \"{filename}\" not found").as_ref()));
    return file.lines().map(Result::unwrap).collect();
}

#[derive(Clone, PartialEq)]
struct Grid {
    blizzards: Vec<Vec<Vec<u8>>>,
    start_pos: (usize, usize),
    end_pos: (usize, usize),
}

// this drops the walls from the input and the start/end pos are +1/-2 in respect to this grid
fn parse(rows: &Vec<String>) -> Grid {
    Grid {
        blizzards: rows.iter().skip(1).rev().skip(1).rev().map(|row| row.as_bytes().iter()
            .filter(|c| **c != b'#')
            .map(|c| if [b'>', b'v', b'<', b'^'].contains(c) { vec!(*c) } else { vec!() })
            .collect::<Vec<_>>()).collect::<Vec<_>>(),
        start_pos: (0, rows.first().unwrap().as_bytes().iter().position(|c| *c == b'.').unwrap()),
        end_pos: (rows.len() - 1, rows.last().unwrap().as_bytes().iter().position(|c| *c == b'.').unwrap()),
    }
}

fn step(grid: &Vec<Vec<Vec<u8>>>) -> Vec<Vec<Vec<u8>>> {
    let mut new_grid = vec![vec![vec!(); grid[0].len()]; grid.len()];

    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, blizzards) in row.iter().enumerate() {
            for &blizzard in blizzards {
                let r = if blizzard == b'^' {
                    row_idx + grid.len() - 1
                } else if blizzard == b'v' {
                    row_idx + 1
                } else {
                    row_idx
                };

                let c = if blizzard == b'<' {
                    col_idx + grid[0].len() - 1
                } else if blizzard == b'>' {
                    col_idx + 1
                } else {
                    col_idx
                };

                new_grid[r % grid.len()][c % grid[0].len()].push(blizzard);
            }
        }
    }

    new_grid
}

fn find_next_positions(grid: &Vec<Vec<Vec<u8>>>, positions: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut next_positions = vec!();

    for pos in positions {
        if grid[pos.0][pos.1].is_empty() {
            next_positions.push(*pos);
        }

        if pos.0 > 0 && grid[pos.0 - 1][pos.1].is_empty() {
            next_positions.push((pos.0 - 1, pos.1));
        }
        if pos.0 + 1 < grid.len() && grid[pos.0 + 1][pos.1].is_empty() {
            next_positions.push((pos.0 + 1, pos.1));
        }

        if pos.1 > 0 && grid[pos.0][pos.1 - 1].is_empty() {
            next_positions.push((pos.0, pos.1 - 1));
        }
        if pos.1 + 1 < grid[0].len() && grid[pos.0][pos.1 + 1].is_empty() {
            next_positions.push((pos.0, pos.1 + 1));
        }
    }

    next_positions.sort_unstable();
    next_positions.dedup();
    next_positions
}

fn part1(rows: &Vec<String>) -> i32 {
    let mut grid = parse(rows);
    let adjusted_start = (grid.start_pos.0, grid.start_pos.1 - 1);
    let adjusted_end = (grid.end_pos.0 - 2, grid.end_pos.1 - 1);

    let mut round = 1;
    let mut positions = vec!();

    while !positions.contains(&adjusted_end) {
        grid.blizzards = step(&grid.blizzards);

        if positions.is_empty() && grid.blizzards[adjusted_start.0][adjusted_start.1].is_empty() {
            positions = vec!(adjusted_start);
        } else {
            positions = find_next_positions(&grid.blizzards, &positions);
        }

        round += 1;
    }

    round
}

fn part2(rows: &Vec<String>) -> usize {
    let mut grid = parse(rows);
    let mut round = 1;

    for walk in 1..=3 {
        let (adjusted_start, adjusted_end) =
            if walk % 2 == 1 {
                ((grid.start_pos.0, grid.start_pos.1 - 1), (grid.end_pos.0 - 2, grid.end_pos.1 - 1))
            } else {
                ((grid.end_pos.0 - 2, grid.end_pos.1 - 1), (grid.start_pos.0, grid.start_pos.1 - 1))
            };

        let mut positions = vec!();
        while !positions.contains(&adjusted_end) {
            grid.blizzards = step(&grid.blizzards);

            if positions.is_empty() && grid.blizzards[adjusted_start.0][adjusted_start.1].is_empty() {
                positions = vec!(adjusted_start);
            } else {
                positions = find_next_positions(&grid.blizzards, &positions);
            }

            round += 1;
        }
    }

    round
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
        assert_eq!(part1(&lines), 18);
        assert_eq!(part2(&lines), 54);
    }
}
