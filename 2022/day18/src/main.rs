use std::cmp::{max, min};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn readlines(filename: &str) -> Vec<String> {
    let file = BufReader::new(File::open(&filename)
        .expect(format!("File \"{filename}\" not found").as_ref()));
    return file.lines().map(Result::unwrap).collect();
}

fn parse(rows: &Vec<String>) -> Vec<[i32; 3]> {
    rows.iter()
        .map(|row| row.split(',').collect::<Vec<_>>())
        .map(|tokens| tokens.iter().map(|t| t.parse().unwrap()).collect::<Vec<_>>())
        .map(|tokens| [tokens[0], tokens[1], tokens[2]])
        .collect()
}

fn initialize_grids(rows: &Vec<String>) -> (Vec<Vec<Vec<bool>>>, Vec<Vec<Vec<bool>>>) {
    let cubes = parse(rows);
    let mut droplets_grid: Vec<Vec<Vec<bool>>>;
    let mut steam_grid: Vec<Vec<Vec<bool>>>;

    let mins = cubes.iter()
        .fold([i32::MAX; 3], |a, b| [min(a[0], b[0]), min(a[1], b[1]), min(a[2], b[2])]);
    let maxs = cubes.iter()
        .fold([0; 3], |a, b| [max(a[0], b[0]), max(a[1], b[1]), max(a[2], b[2])]);
    let dims = [(maxs[0] - mins[0] + 1) as usize, (maxs[1] - mins[1] + 1) as usize, (maxs[2] - mins[2] + 1) as usize];

    droplets_grid = vec![vec![vec![false; dims[2]]; dims[1]]; dims[0]];
    steam_grid = droplets_grid.clone();

    for x in 0..dims[0] {
        for y in 0..dims[1] {
            for z in 0..dims[2] {
                steam_grid[x][y][z] = x == 0 || x == dims[0] - 1
                    || y == 0 || y == dims[1] - 1
                    || z == 0 || z == dims[2] - 1;
            }
        }
    }

    for cube in &cubes {
        let pos = [(cube[0] - mins[0]) as usize, (cube[1] - mins[1]) as usize, (cube[2] - mins[2]) as usize];
        droplets_grid[pos[0]][pos[1]][pos[2]] = true;
        // not really needed but it may get confusing, if a cube is a droplet and steam at the same time
        steam_grid[pos[0]][pos[1]][pos[2]] = false;
    }

    (droplets_grid, steam_grid)
}

fn count_uncovered_sides<P>(droplets_grid: &Vec<Vec<Vec<bool>>>, predicate: P) -> u64
    where P: Fn(&[usize; 3]) -> bool
{
    // assumes we've a cube
    let dims = [droplets_grid.len(), droplets_grid[0].len(), droplets_grid[0][0].len()];
    let mut count = 0;

    for x in 0..dims[0] {
        for y in 0..dims[1] {
            for z in 0..dims[2] {
                if !droplets_grid[x][y][z] {
                    continue;
                }

                for idx in 0..3 {
                    for step in [-1isize, 1] {
                        let mut pos = [x, y, z];
                        if step < 0 && pos[idx] == 0 {
                            count += 1;
                        } else {
                            pos[idx] = if step < 0 { pos[idx] - step.abs() as usize } else { pos[idx] + step as usize };
                            if pos[idx] == dims[idx] || predicate(&pos) {
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    count
}

fn part1(rows: &Vec<String>) -> u64 {
    let (droplets_grid, _) = initialize_grids(rows);
    count_uncovered_sides(&droplets_grid, |pos| !droplets_grid[pos[0]][pos[1]][pos[2]])
}

fn part2(rows: &Vec<String>) -> u64 {
    let (droplets_grid, mut steam_grid) = initialize_grids(rows);

    // propagate the steam
    let mut changed = true;
    while changed {
        changed = false;
        for x in 1..steam_grid.len() - 1 {
            for y in 1..steam_grid[x].len() - 1 {
                for z in 1..steam_grid[x][y].len() - 1 {
                    if droplets_grid[x][y][z] || steam_grid[x][y][z] {
                        continue;
                    }

                    for idx in 0..=2 {
                        for step in [-1isize, 1] {
                            let mut pos = [x, y, z];
                            pos[idx] = if step < 0 { pos[idx] - step.abs() as usize } else { pos[idx] + step as usize };
                            if !droplets_grid[pos[0]][pos[1]][pos[2]] && steam_grid[pos[0]][pos[1]][pos[2]] {
                                steam_grid[x][y][z] = true;
                                changed = true;
                            }
                        }
                    }
                }
            }
        }
    }

    count_uncovered_sides(&droplets_grid,
                          |pos| !droplets_grid[pos[0]][pos[1]][pos[2]] && steam_grid[pos[0]][pos[1]][pos[2]])
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
        assert_eq!(part1(&lines), 64);
        assert_eq!(part2(&lines), 58);
    }
}
