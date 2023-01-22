use std::cmp::max;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn readlines(filename: &str) -> Vec<String> {
    let file = BufReader::new(File::open(&filename)
        .expect(format!("File \"{filename}\" not found").as_ref()));
    return file.lines().map(Result::unwrap).collect();
}

struct Tree {
    height: u8,
    visible: bool,
}

struct Position2(usize, usize);

struct Movement2(isize, isize);

impl Position2 {
    fn translate(a: &Position2, b: &Movement2) -> Self {
        Position2(if b.0 >= 0 { a.0 + b.0 as usize } else { a.0 - b.0.abs() as usize },
                  if b.1 >= 1 { a.1 + b.1 as usize } else { a.1 - b.1.abs() as usize })
    }
}

fn parse(rows: &Vec<String>) -> Vec<Vec<u8>> {
    rows.iter()
        .map(|row| row.chars().collect::<Vec<char>>())
        .map(|chars| chars.iter().map(|c| *c as u8 - '0' as u8).collect())
        .collect()
}

fn find_first_equal_or_larger_in_sight(grid: &Vec<Vec<Tree>>, from: &Position2, movement: &Movement2) -> Option<Position2> {
    let max_height = grid[from.0][from.1].height;
    let mut pos: Position2 = Position2 { ..*from };

    while (movement.0 > 0 || pos.0 >= movement.0.abs() as usize)
        && (movement.1 > 0 || pos.1 >= movement.1.abs() as usize) {
        pos = Position2::translate(&pos, movement);
        match grid.get(pos.0).and_then(|row| row.get(pos.1)).map(|tree| tree.height) {
            Some(height) => if height >= max_height {
                return Some(Position2 { ..pos });
            },
            None => break
        }
    }
    None
}

fn part1(rows: &Vec<String>) -> usize {
    let mut grid: Vec<Vec<Tree>> = parse(rows).iter()
        .map(|row| row.iter()
            .map(|height| Tree { height: *height, visible: false }).collect::<Vec<Tree>>())
        .collect();

    for row_idx in 0..grid.len() {
        for col_idx in 0..grid[row_idx].len() {
            let pos = Position2(row_idx, col_idx);
            for movement in [Movement2(-1, 0), Movement2(0, -1), Movement2(0, 1), Movement2(1, 0)] {
                grid[pos.0][pos.1].visible = match find_first_equal_or_larger_in_sight(&grid, &pos, &movement) {
                    Some(_) => false,
                    None => true
                };
                if grid[pos.0][pos.1].visible {
                    break;
                }
            }
        }
    }
    grid.iter().map(|row| row.iter().filter(|tree| tree.visible).count()).sum()
}

fn part2(rows: &Vec<String>) -> usize {
    let grid: Vec<Vec<Tree>> = parse(rows).iter()
        .map(|row| row.iter()
            .map(|height| Tree { height: *height, visible: false }).collect::<Vec<Tree>>())
        .collect();

    let mut score = usize::MIN;

    for row_idx in 1..grid.len() - 1 {
        for col_idx in 1..grid[row_idx].len() - 1 {
            let mut crnt_score = 1usize;
            let pos = Position2(row_idx, col_idx);
            for movement in [Movement2(-1, 0), Movement2(0, -1), Movement2(0, 1), Movement2(1, 0)] {
                crnt_score *= match find_first_equal_or_larger_in_sight(&grid, &pos, &movement) {
                    Some(highest_pos) => match movement {
                        Movement2(_, 0) => pos.0.abs_diff(highest_pos.0),
                        Movement2(0, _) => pos.1.abs_diff(highest_pos.1),
                        _ => panic!("Ouch")
                    }
                    None => match movement {
                        Movement2(step, 0) => if step < 0 { row_idx } else { grid.len() - row_idx - 1 },
                        Movement2(0, step) => if step < 0 { col_idx } else { grid[pos.0].len() - col_idx - 1 },
                        _ => panic!("Ouch")
                    }
                }
            }
            score = max(score, crnt_score);
        }
    }
    score
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
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
        assert_eq!(part1(&lines), 21);
        assert_eq!(part2(&lines), 8);
    }
}
