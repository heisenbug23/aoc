use std::fs::File;
use std::io::{BufRead, BufReader};

fn readlines(filename: &str) -> Vec<String> {
    let file = BufReader::new(File::open(&filename)
        .expect(format!("File \"{filename}\" not found").as_ref()));
    return file.lines().map(Result::unwrap).collect();
}

fn buckets(rows: &Vec<String>) -> Vec<i32> {
    let mut buckets: Vec<i32> = vec!(0);
    rows.iter().for_each(|row| match row.as_str() {
        "" => buckets.push(0),
        some => {
            let current = buckets.pop().unwrap();
            buckets.push(current + some.parse::<i32>().unwrap());
        }
    });
    return buckets;
}

fn part1(rows: &Vec<String>) -> i32 {
    return buckets(rows).iter().max().expect("Ouch").clone();
}

fn part2(rows: &Vec<String>) -> i32 {
    let mut values = buckets(rows);
    values.sort();
    return values.iter().rev().take(3).sum();
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
        assert_eq!(part1(&lines), 24000);
        assert_eq!(part2(&lines), 45000);
    }
}
