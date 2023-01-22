use std::collections::{HashMap, VecDeque};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn readlines(filename: &str) -> Vec<String> {
    let file = BufReader::new(File::open(&filename)
        .expect(format!("File \"{filename}\" not found").as_ref()));
    return file.lines().map(Result::unwrap).collect();
}

fn parse(rows: &Vec<String>) -> Vec<(i32, i32)> {
    let mut elves = vec!();

    for (row_idx, row) in rows.iter().enumerate() {
        for (col_idx, col) in row.as_bytes().iter().enumerate() {
            if *col == b'#' {
                elves.push((row_idx as i32, col_idx as i32));
            }
        }
    }

    elves
}

struct Grid {
    data: Vec<Vec<bool>>,
    row_offset: i32,
    col_offset: i32,
}

impl Grid {
    fn from(positions: &Vec<(i32, i32)>) -> Self {
        let min_row = positions.iter().map(|p| p.0).min().unwrap();
        let max_row = positions.iter().map(|p| p.0).max().unwrap();
        let min_col = positions.iter().map(|p| p.1).min().unwrap();
        let max_col = positions.iter().map(|p| p.1).max().unwrap();

        let mut data = vec![vec![false; (max_col - min_col) as usize + 1]; (max_row - min_row) as usize + 1];

        for pos in positions {
            data[(pos.0 - min_row) as usize][(pos.1 - min_col) as usize] = true;
        }

        Grid {
            data,
            row_offset: min_row,
            col_offset: min_col,
        }
    }

    fn is_occupied(&self, pos: &(i32, i32)) -> bool {
        let p = ((pos.0 - self.row_offset) as usize, (pos.1 - self.col_offset) as usize);
        self.data.get(p.0).and_then(|row| row.get(p.1)).get_or_insert(&false).clone()
    }
}

fn has_neighbors(grid: &Grid, pos: &(i32, i32)) -> bool {
    for row in -1..=1i32 {
        for col in -1..=1i32 {
            if row != 0 || col != 0 {
                if grid.is_occupied(&(pos.0 + row, pos.1 + col)) {
                    return true;
                }
            }
        }
    }
    false
}

fn find_proposals(elves: &Vec<(i32, i32)>, directions: &Vec<u8>) -> Vec<Option<(i32, i32)>> {
    let grid = Grid::from(elves);
    let mut proposals: Vec<Option<(i32, i32)>> = vec!();
    proposals.reserve(elves.len());

    for pos in elves {
        let mut dest = None;

        if has_neighbors(&grid, pos) {
            for &dir in directions {
                let (new_pos, to_check) =
                    if dir == b'N' || dir == b'S' {
                        let step = if dir == b'N' { -1 } else { 1 };
                        (
                            (pos.0 + step, pos.1),
                            [(pos.0 + step, pos.1 - 1), (pos.0 + step, pos.1), (pos.0 + step, pos.1 + 1)]
                        )
                    } else {
                        let step = if dir == b'W' { -1 } else { 1 };
                        (
                            (pos.0, pos.1 + step),
                            [(pos.0 - 1, pos.1 + step), (pos.0, pos.1 + step), (pos.0 + 1, pos.1 + step)]
                        )
                    };

                if !to_check.iter().any(|p| grid.is_occupied(p)) {
                    dest = Some(new_pos);
                    break;
                }
            }
        }

        proposals.push(dest);
    }

    proposals
}

fn do_round(elves: &mut Vec<(i32, i32)>, directions: &Vec<u8>) -> bool {
    let proposals = find_proposals(elves, directions);
    let mut counts: HashMap<(i32, i32), i32> = HashMap::new();

    proposals.iter().for_each(|p| if let Some(pos) = p {
        counts.insert(pos.clone(), *counts.get(pos).unwrap_or(&0) + 1);
    });

    let mut changed = false;
    proposals.iter().enumerate().for_each(|(idx, p)| if let Some(pos) = p {
        if *counts.get(pos).unwrap() == 1 {
            elves[idx] = *pos;
            changed = true;
        }
    });

    changed
}

fn part1(rows: &Vec<String>) -> usize {
    let mut elves = parse(rows);
    let mut directions = VecDeque::from([b'N', b'S', b'W', b'E']);

    for _ in 0..10 {
        do_round(&mut elves, &directions.iter().cloned().collect());
        let tmp = directions.pop_front().unwrap();
        directions.push_back(tmp);
    }

    Grid::from(&elves).data.iter().map(|row| row.iter().filter(|p| !**p).count()).sum()
}

fn part2(rows: &Vec<String>) -> usize {
    let mut elves = parse(rows);
    let mut directions = VecDeque::from([b'N', b'S', b'W', b'E']);

    let mut rounds = 1;
    while do_round(&mut elves, &directions.iter().cloned().collect()) {
        rounds += 1;
        let tmp = directions.pop_front().unwrap();
        directions.push_back(tmp);
    }

    rounds
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
        assert_eq!(part1(&lines), 110);
        assert_eq!(part2(&lines), 20);
    }
}
