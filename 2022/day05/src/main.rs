use std::fs::File;
use std::io::{BufRead, BufReader};

struct Procedure {
    num: usize,
    from: usize,
    to: usize,
}

fn readlines(filename: &str) -> Vec<String> {
    let file = BufReader::new(File::open(&filename)
        .expect(format!("File \"{filename}\" not found").as_ref()));
    return file.lines().map(Result::unwrap).collect();
}

fn parse(rows: &Vec<String>) -> (Vec<Vec<char>>, Vec<Procedure>) {
    let mut parsed_crate_lines: Vec<Vec<char>> = vec!();
    let mut procedures: Vec<Procedure> = vec!();
    let mut mode = "PARSE_STACK";

    for row in rows {
        if mode == "PARSE_STACK" {
            if row.trim() == "" {
                parsed_crate_lines.pop();
                mode = "PARSE_PROCEDURE";
            } else {
                parsed_crate_lines.push(row.chars().into_iter()
                    .skip(1)
                    .step_by(4)
                    .collect())
            }
        } else {
            let tokens = row.split(' ').collect::<Vec<&str>>();
            procedures.push(Procedure {
                num: tokens[1].parse::<usize>().expect("Ouch"),
                from: tokens[3].parse::<usize>().expect("Ouch") - 1,
                to: tokens[5].parse::<usize>().expect("Ouch") - 1,
            });
        }
    }

    let mut stacks: Vec<Vec<char>> = vec!();
    for _ in 0..parsed_crate_lines.last().unwrap().len() {
        stacks.push(vec!());
    }

    for line in parsed_crate_lines.iter().rev() {
        for (idx, container) in line.iter().enumerate() {
            if *container != ' ' {
                stacks[idx].push(*container);
            }
        }
    }

    (stacks, procedures)
}

fn doit<E>(rows: &Vec<String>, extender: E) -> String
    where E: Fn(&mut Vec<char>, &mut Vec<char>)
{
    let (mut stacks, procedures): (Vec<Vec<char>>, Vec<Procedure>) = parse(rows);
    for proc in procedures {
        let mut to_move: Vec<char> = vec!();

        let wanted_len = stacks[proc.from].len() - proc.num;
        for element in stacks[proc.from].drain(wanted_len..) {
            to_move.push(element);
        }

        extender(&mut stacks[proc.to], &mut to_move);
    }

    return stacks.iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>();
}

fn part1(rows: &Vec<String>) -> String {
    return doit(rows, |stack, values| { stack.extend(values.iter().rev()) });
}

fn part2(rows: &Vec<String>) -> String {
    return doit(rows, |stack, values| { stack.extend(values.iter()) });
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
        assert_eq!(part1(&lines), "CMZ");
        assert_eq!(part2(&lines), "MCD");
    }
}
