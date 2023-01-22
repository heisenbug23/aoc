use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn readlines(filename: &str) -> Vec<String> {
    let file = BufReader::new(File::open(&filename)
        .expect(format!("File \"{filename}\" not found").as_ref()));
    return file.lines().map(Result::unwrap).collect();
}

// TODO How to own a closure in a struct?
//  This would help parsing the op only once per monkey.
#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    operation: Vec<String>,
    test_mod: usize,
    throw_true: usize,
    throw_false: usize,
}

fn parse(rows: &Vec<String>) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = vec!();

    for monkey_desc in rows.chunks(7) {
        let which = monkey_desc[0].split(' ').last().unwrap().trim_matches(':').parse::<usize>().unwrap();
        assert_eq!(which, monkeys.len());

        monkeys.push(Monkey {
            items: monkey_desc[1].trim().split(':').last().unwrap().split(',').map(|x| x.trim().parse().unwrap()).collect(),
            operation: monkey_desc[2].trim().split(' ').skip(3).map(|s| s.to_string()).collect(),
            test_mod: monkey_desc[3].trim().split(' ').last().unwrap().parse().unwrap(),
            throw_true: monkey_desc[4].trim().split(' ').last().unwrap().parse().unwrap(),
            throw_false: monkey_desc[5].trim().split(' ').last().unwrap().parse().unwrap(),
        });
    }

    monkeys
}

fn compute_new_level(old: usize, op_tokens: &Vec<String>) -> usize {
    let left = if op_tokens[0] == "old" { old } else { op_tokens[0].parse().unwrap() };
    let right = if op_tokens[2] == "old" { old } else { op_tokens[2].parse().unwrap() };
    match op_tokens[1].as_str() {
        "+" => left + right,
        "-" => left - right,
        "*" => left * right,
        "/" => left / right,
        op => panic!("Unknown op {}", format!("{op}"))
    }
}

fn doit<F>(monkeys: &mut Vec<Monkey>, num_rounds: usize, update_lvl: F) -> usize
    where F: Fn(usize) -> usize
{
    let mut num_inspections = vec![0; monkeys.len()];

    for _ in 0..num_rounds {
        for monkey_idx in 0..monkeys.len() {
            for item in monkeys[monkey_idx].items.clone() {
                num_inspections[monkey_idx] += 1;
                let monkey = &monkeys[monkey_idx];
                let mut lvl: usize = item;
                lvl = compute_new_level(lvl, &monkey.operation);
                lvl = update_lvl(lvl);
                let dest = if lvl % monkey.test_mod == 0 {
                    monkey.throw_true
                } else {
                    monkey.throw_false
                };
                monkeys[dest].items.push(lvl);
            }
            monkeys[monkey_idx].items.clear();
        }
    }

    num_inspections.sort();
    num_inspections.iter().rev().take(2).fold(1, |a, b| a * *b)
}

fn part1(rows: &Vec<String>) -> usize {
    let mut monkeys = parse(rows);
    doit(&mut monkeys, 20, |lvl| lvl / 3)
}

fn part2(rows: &Vec<String>) -> usize {
    let mut monkeys = parse(rows);
    // jep, that may not really be the lcm, but it's good enough
    let lcm = monkeys.iter().map(|m| m.test_mod).fold(1, |a, b| a * b);
    doit(&mut monkeys, 10000, |lvl| lvl % lcm)
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
        assert_eq!(part1(&lines), 10605);
        assert_eq!(part2(&lines), 2713310158);
    }
}
