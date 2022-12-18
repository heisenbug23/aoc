use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn readlines(filename: &str) -> Vec<String> {
    let file = BufReader::new(File::open(&filename)
        .expect(format!("File \"{filename}\" not found").as_ref()));
    return file.lines().map(Result::unwrap).collect();
}

fn parse(rows: &Vec<String>) -> Vec<((usize, usize), (usize, usize))> {
    rows.iter()
        .map(|row| row.split("->").collect::<Vec<&str>>())
        .flat_map(|path| {
            let points: Vec<(usize, usize)> = path.iter()
                .map(|p| p.trim().split(',').collect::<Vec<&str>>())
                .map(|split| (split[0].parse().unwrap(), split[1].parse().unwrap()))
                .collect();
            points.iter().zip(points.iter().skip(1))
                .map(|(&a, &b)| (a, b))
                .collect::<Vec<((usize, usize), (usize, usize))>>()
        })
        .collect()
}

fn render_rocks(lines: &Vec<((usize, usize), (usize, usize))>) -> Vec<(usize, usize)> {
    lines.iter()
        .flat_map(|(start, end)| {
            let rows = if start.1 <= end.1 { start.1..=end.1 } else { end.1..=start.1 };
            let cols = if start.0 <= end.0 { start.0..=end.0 } else { end.0..=start.0 };
            rows.map(|row| cols.clone().map(|col| (col, row)).collect::<Vec<(usize, usize)>>())
                .flatten()
                .collect::<Vec<(usize, usize)>>()
        })
        .collect()
}

fn doit(lines: &Vec<((usize, usize), (usize, usize))>) -> usize {
    let rocks: HashSet<(usize, usize)> = HashSet::from_iter(render_rocks(&lines));
    let mut resting: HashSet<(usize, usize)> = HashSet::new();
    let max_y = lines.iter().flat_map(|n| [n.0.1, n.1.1]).max().unwrap();

    let mut prev_num_resting = usize::MAX;
    while prev_num_resting != resting.len() {
        prev_num_resting = resting.len();

        let mut pos = (500, 0);
        while pos.1 <= max_y {
            let mut moved = false;
            for dir in [(0isize, 1isize), (-1, 1), (1, 1)] {
                let new_pos = (
                    if dir.0 >= 0 { pos.0 + dir.0 as usize } else { pos.0 - dir.0.abs() as usize },
                    if dir.1 >= 0 { pos.1 + dir.1 as usize } else { pos.1 - dir.1.abs() as usize },
                );

                if rocks.contains(&new_pos) || resting.contains(&new_pos) {
                    continue;
                }

                pos = new_pos;
                moved = true;
                break;
            }

            if !moved {
                resting.insert(pos);
                break;
            }
        }
    }

    resting.len()
}

fn part1(rows: &Vec<String>) -> usize {
    doit(&parse(rows))
}

fn part2(rows: &Vec<String>) -> usize {
    let mut lines = parse(rows);
    let max_y = lines.iter().flat_map(|n| [n.0.1, n.1.1]).max().unwrap() + 2;
    lines.push(((0, max_y), (10000, max_y)));
    doit(&lines)
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
        assert_eq!(part1(&lines), 24);
        assert_eq!(part2(&lines), 93);
    }
}
