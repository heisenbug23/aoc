use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::Job::*;

fn readlines(filename: &str) -> Vec<String> {
    let file = BufReader::new(File::open(&filename)
        .expect(format!("File \"{filename}\" not found").as_ref()));
    return file.lines().map(Result::unwrap).collect();
}

#[derive(Debug, Clone)]
enum Job {
    Yell(i64),
    Op(String, String, u8),
}

fn parse(rows: &Vec<String>) -> HashMap<String, Job> {
    rows.iter().map(|line| {
        let token = line.split(':').collect::<Vec<_>>();
        let name = token[0];
        let rhs_token = token[1].trim().split(' ').collect::<Vec<_>>();
        if rhs_token.len() == 1 {
            (name.to_string(), Yell(rhs_token[0].parse().unwrap()))
        } else {
            (name.to_string(), Op(rhs_token[0].to_string(),
                                  rhs_token[2].to_string(),
                                  rhs_token[1].as_bytes()[0]))
        }
    }).collect()
}

fn process_until_unchanged(monkeys: &mut HashMap<String, Job>) {
    let mut changed = true;
    while changed {
        changed = false;

        for to_check in monkeys.iter()
            .filter(|(_, job)| match job {
                Op(_, _, _) => true,
                _ => false
            })
            .map(|(name, _)| name)
            .cloned()
            .collect::<Vec<_>>()
        {
            let Op(lhs, rhs, op) = monkeys.get(&to_check).unwrap() else { panic!() };

            let left = if let Some(Yell(number)) = monkeys.get(lhs) { Some(number) } else { None };
            let right = if let Some(Yell(number)) = monkeys.get(rhs) { Some(number) } else { None };

            if let Some(result) = left.zip(right).and_then(|(l, &r)| match op {
                b'+' => l.checked_add(r),
                b'-' => l.checked_sub(r),
                b'*' => l.checked_mul(r),
                b'/' => l.checked_div(r),
                _ => panic!()
            }) {
                monkeys.insert(to_check, Yell(result));
                changed = true;
            }
        }
    }
}

fn find_diff_for_yell(monkeys: &HashMap<String, Job>, to_yell: i64) -> Option<i64> {
    let mut monkeys = monkeys.clone();
    let Op(root_lhs, root_rhs, _) = monkeys.get("root").cloned().unwrap() else { panic!() };
    monkeys.insert("humn".to_string(), Yell(to_yell));
    process_until_unchanged(&mut monkeys);

    monkeys.get(&root_lhs).zip(monkeys.get(&root_rhs)).and_then(|jobs| match jobs {
        (Yell(left), Yell(right)) => Some(left - right),
        _ => None
    })
}

fn find_it(monkeys: &HashMap<String, Job>,
           lower_bound: i64,
           upper_bound: i64) -> Option<i64> {
    let half = lower_bound + (upper_bound - lower_bound) / 2;

    if let Some(diff) = find_diff_for_yell(monkeys, half) {
        return if diff == 0 {
            Some(half)
        } else if diff < 0 {
            find_it(monkeys, half + 1, upper_bound)
        } else {
            find_it(monkeys, lower_bound, half)
        }
    }

    None
}

fn find_bounds(monkeys: &HashMap<String, Job>) -> (i64, i64) {
    let lower_limit = 0i64;
    let mut upper_limit = i64::MAX;

    let mut lower = find_diff_for_yell(&monkeys, lower_limit);

    let mut upper = find_diff_for_yell(&monkeys, upper_limit);
    let mut shift = 1;
    while upper.is_none() {
        upper_limit = i64::MAX >> shift;
        shift += 1;
        upper = find_diff_for_yell(&monkeys, upper_limit);
    }

    if lower.unwrap() <= upper.unwrap() {
        (lower_limit, upper_limit)
    } else {
        (upper_limit, lower_limit)
    }
}

fn part1(rows: &Vec<String>) -> i64 {
    let mut monkeys = parse(rows);
    process_until_unchanged(&mut monkeys);

    if let Some(Yell(number)) = monkeys.get("root") {
        return *number;
    } else {
        panic!("Didn't find a solution");
    }
}

fn part2(rows: &Vec<String>) -> i64 {
    let mut monkeys = parse(rows);
    monkeys.insert("humn".to_string(),Op("not existing".to_string(),
                                         "not existing".to_string(), b'+'));
    process_until_unchanged(&mut monkeys);
    let monkeys = monkeys;

    let (lower_bound, upper_bound) = find_bounds(&monkeys);

    if let Some(mut result) = find_it(&monkeys, lower_bound, upper_bound) {
        while let Some(0) = find_diff_for_yell(&monkeys, result - 1) {
            result -= 1;
        }
        return result;
    }

    panic!("Didn't find a solution");
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
        assert_eq!(part1(&lines), 152);
        assert_eq!(part2(&lines), 301);
    }
}
