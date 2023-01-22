use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn readlines(filename: &str) -> Vec<String> {
    let file = BufReader::new(File::open(&filename)
        .expect(format!("File \"{filename}\" not found").as_ref()));
    return file.lines().map(Result::unwrap).collect();
}

fn priority(a: char) -> u32 {
    if a.is_uppercase() {
        a as u32 - 'A' as u32 + 27
    } else {
        a as u32 - 'a' as u32 + 1
    }
}

// TODO How to do it without cloning the characters all the time?
// TODO What's the proper way to implement variadic functions in Rust?
fn intersect(first: &str, others: &[&str]) -> Vec<char> {
    let mut set: HashSet<char> = HashSet::from_iter(first.chars());
    for &other in others {
        set = set.intersection(&HashSet::from_iter(other.chars()))
            .map(|c| c.clone())
            .collect();
    }
    set.iter().map(|c| c.clone()).collect::<Vec<char>>()
}

fn part1(rows: &Vec<String>) -> u32 {
    let mut sum: u32 = 0;
    for row in rows {
        let left_right = row.split_at(row.len() / 2);
        let common = intersect(left_right.0, &[left_right.1]);
        assert_eq!(common.len(), 1);
        sum += priority(*common.get(0).expect("Ouch"));
    }
    return sum;
}

fn part2(rows: &Vec<String>) -> u32 {
    let mut sum: u32 = 0;
    for group in rows.iter().step_by(3)
        .zip(rows.iter().skip(1).step_by(3)
            .zip(rows.iter().skip(2).step_by(3))) {
        let common = intersect(group.0, &[group.1.0, group.1.1]);
        assert_eq!(common.len(), 1);
        sum += priority(*common.get(0).expect("Ouch"));
    }
    return sum;
}

fn main() {
    let lines = readlines("input");
    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let lines = readlines("test.in");
        assert_eq!(part1(&lines), 157);
        assert_eq!(part2(&lines), 70);
    }
}
