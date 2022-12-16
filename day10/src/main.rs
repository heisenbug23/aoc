use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn readlines(filename: &str) -> Vec<String> {
    let file = BufReader::new(File::open(&filename)
        .expect(format!("File \"{filename}\" not found").as_ref()));
    return file.lines().map(Result::unwrap).collect();
}

enum Op {
    NOOP,
    ADDX(i32),
}

fn parse(rows: &Vec<String>) -> Vec<Op> {
    rows.iter()
        .map(|row| row.split(' ').collect::<Vec<&str>>())
        .map(|tokens| match tokens[0] {
            "noop" => Op::NOOP,
            "addx" => Op::ADDX(tokens[1].parse().unwrap()),
            other => panic!("{}", format!("Unknown command {other}"))
        })
        .collect()
}

fn process<C>(ops: &Vec<Op>, num_cycles_to_run: i64, mut cycle_callback: C)
    where C: FnMut(i64, i64) {

    let mut reg_x: i64 = 1;
    let mut cycle: i64 = 0;

    for op in ops {
        let num_cycles = match op {
            Op::NOOP => 1,
            Op::ADDX(_) => 2
        };
        for _ in 0..num_cycles {
            cycle += 1;
            cycle_callback(cycle, reg_x);
        }
        match op {
            Op::NOOP => (),
            Op::ADDX(summand) => reg_x += *summand as i64
        }
        if cycle >= num_cycles_to_run {
            break;
        }
    }
}

fn part1(rows: &Vec<String>) -> i64 {
    let ops = parse(rows);
    let mut sum_signal_strengths: i64 = 0;

    process(&ops, 220,|cycle, reg_x| {
        if cycle == 20 || (cycle + 20) % 40 == 0 {
            sum_signal_strengths += reg_x * cycle;
        }
    });

    sum_signal_strengths
}

fn part2(rows: &Vec<String>) {
    let ops = parse(rows);

    process(&ops, 240,|cycle, reg_x| {
        print!("{}", if (reg_x - (cycle % 40 - 1)).abs() <= 1 { "#" } else { " " });
        if cycle % 40 == 0 {
            println!()
        }
    });
    println!();
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let filename = if args.len() == 2 { &args[1] } else { "input" };
    let lines = readlines(filename);
    println!("Part 1: {}", part1(&lines));
    println!("Part 2:");
    part2(&lines);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let lines = readlines("test.in");
        assert_eq!(part1(&lines), 13140);
        part2(&lines); // to be checked manually
    }
}
