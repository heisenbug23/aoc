use std::cmp::max;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn readlines(filename: &str) -> Vec<String> {
    let file = BufReader::new(File::open(&filename)
        .expect(format!("File \"{filename}\" not found").as_ref()));
    return file.lines().map(Result::unwrap).collect();
}

enum Movement {
    D(u32),
    L(u32),
    R(u32),
    U(u32),
}

fn parse(rows: &Vec<String>) -> Vec<Movement> {
    rows.iter()
        .map(|row| row.split(' ').collect::<Vec<&str>>())
        .map(|t| (t[0], t[1].parse::<u32>().unwrap()))
        .map(|t| match t.0 {
            "D" => Movement::D(t.1),
            "L" => Movement::L(t.1),
            "R" => Movement::R(t.1),
            "U" => Movement::U(t.1),
            _ => panic!("Ouch, unexpected input")
        }).collect()
}

fn add(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    (a.0 + b.0, a.1 + b.1)
}

fn diff(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    (a.0 - b.0, a.1 - b.1)
}

fn find_num_visits_of_tail(movements: &Vec<Movement>, mut knots: Vec<(i32, i32)>) -> usize {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert(*knots.last().unwrap());

    for movement in movements {
        let (&steps, step_vector) = match movement {
            Movement::D(steps) => (steps, (0, -1)),
            Movement::L(steps) => (steps, (-1, 0)),
            Movement::R(steps) => (steps, (1, 0)),
            Movement::U(steps) => (steps, (0, 1))
        };
        for _ in 0..steps {
            knots[0] = add(knots[0], step_vector);

            for idx in 1..knots.len() {
                let diff = diff(knots[idx - 1], knots[idx]);
                if diff.0.abs() > 1 || diff.1.abs() > 1 {
                    knots[idx] = add(knots[idx], (diff.0 / max(diff.0.abs(), 1), diff.1 / max(diff.1.abs(), 1)));
                }
            }

            visited.insert(*knots.last().unwrap());
        }
    }

    visited.len()
}

fn part1(rows: &Vec<String>) -> usize {
    find_num_visits_of_tail(&parse(rows), vec![(0, 0); 2])
}

fn part2(rows: &Vec<String>) -> usize {
    find_num_visits_of_tail(&parse(rows), vec![(0, 0); 10])
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
        assert_eq!(part1(&lines), 13);
        assert_eq!(part2(&lines), 1);
        let lines = readlines("test2.in");
        assert_eq!(part2(&lines), 36);
    }
}
