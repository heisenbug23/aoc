use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn readlines(filename: &str) -> Vec<String> {
    let file = BufReader::new(File::open(&filename)
        .expect(format!("File \"{filename}\" not found").as_ref()));
    return file.lines().map(Result::unwrap).collect();
}

struct Node {
    number: i64,
    order: usize,
}

fn parse(rows: &Vec<String>) -> Vec<Node> {
    rows.iter().enumerate().map(|(order, s)| Node {
        number: s.parse().unwrap(),
        order
    }).collect::<Vec<_>>()
}

fn mix(numbers: &mut Vec<Node>) {
    for order in 0..numbers.len() {
        let mut idx = numbers.iter().position(|n| n.order == order).unwrap();
        let number = numbers[idx].number;

        let mut num_steps = number.abs() as usize;
        while num_steps >= numbers.len() {
            num_steps = num_steps / numbers.len() + num_steps % numbers.len();
        }

        for _ in 0..num_steps {
            let idx_b = if number >= 0 { idx + 1 } else { idx + numbers.len() - 1 } % numbers.len();
            numbers.swap(idx, idx_b);
            idx = idx_b;
        }
    }
}

fn sum_grove_coordinates(nodes: &Vec<Node>) -> i64 {
    let zero_idx = nodes.iter().position(|n| n.number == 0).unwrap();
    nodes[(zero_idx + 1000) % nodes.len()].number
        + nodes[(zero_idx + 2000) % nodes.len()].number
        + nodes[(zero_idx + 3000) % nodes.len()].number
}

fn part1(rows: &Vec<String>) -> i64 {
    let mut nodes = parse(rows);
    mix(&mut nodes);
    sum_grove_coordinates(&nodes)
}

fn part2(rows: &Vec<String>) -> i64 {
    let mut nodes = parse(rows);
    nodes.iter_mut().for_each(|mut n| n.number *= 811589153i64);

    (0..10).for_each(|_| mix(&mut nodes));
    sum_grove_coordinates(&nodes)
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
        assert_eq!(part1(&lines), 3);
        assert_eq!(part2(&lines), 1623178306);
    }
}
