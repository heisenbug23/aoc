use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn readlines(filename: &str) -> Vec<String> {
    let file = BufReader::new(File::open(&filename)
        .expect(format!("File \"{filename}\" not found").as_ref()));
    return file.lines().map(Result::unwrap).collect();
}

fn split_to_vector<T: FromStr>(to_split: &str, delimiter: char) -> Vec<T>
    where <T as FromStr>::Err: Debug
{
    to_split.split(delimiter)
        .map(|x| x.parse::<T>().expect("Ouch"))
        .collect::<Vec<T>>()
}

fn count<P>(rows: &Vec<String>, predicate: P) -> u32
    where P: Fn((u32, u32), (u32, u32)) -> bool
{
    let mut count: u32 = 0;
    for row in rows {
        let split = row.split(',').collect::<Vec<&str>>();
        let left = split_to_vector::<u32>(split[0], '-');
        let right = split_to_vector::<u32>(split[1], '-');
        if predicate((left[0], left[1]), (right[0], right[1])) {
            count += 1
        }
    }
    return count;
}

fn part1(rows: &Vec<String>) -> u32 {
    count(rows, |left, right|
        (left.0 <= right.0 && right.0 <= right.1 && right.1 <= left.1)
            || (right.0 <= left.0 && left.0 <= left.1 && left.1 <= right.1),
    )
}

fn part2(rows: &Vec<String>) -> u32 {
    count(rows, |left, right|
        (left.0 <= right.0 && right.0 <= left.1)
            || (right.0 <= left.0 && left.0 <= right.1),
    )
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
        assert_eq!(part1(&lines), 2);
        assert_eq!(part2(&lines), 4);
    }
}
