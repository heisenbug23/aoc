use std::cmp::Ordering;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn readlines(filename: &str) -> Vec<String> {
    let file = BufReader::new(File::open(&filename)
        .expect(format!("File \"{filename}\" not found").as_ref()));
    return file.lines().map(Result::unwrap).collect();
}

fn find_token_end(data: &[u8], start: usize) -> usize {
    let mut pos = start;

    if data[pos] == '[' as u8 {
        pos = pos + 1;
        while data[pos] != ']' as u8 {
            if data[pos] == '[' as u8 {
                pos = find_token_end(data, pos) + 1;
            } else {
                pos += 1;
            }
        }
    } else {
        while pos < data.len() && data[pos] != ',' as u8 {
            pos += 1;
        }
    }

    pos
}

fn is_in_order(left: &[u8], right: &[u8]) -> Option<bool> {
    let mut l: usize = 0;
    let mut r: usize = 0;

    while l < left.len() && r < right.len() {
        if left[l] == ',' as u8 {
            l += 1;
        }
        if right[r] == ',' as u8 {
            r += 1;
        }

        let lstart = if left[l] == '[' as u8 { l + 1 } else { l };
        let rstart = if right[r] == '[' as u8 { r + 1 } else { r };
        let lend = find_token_end(left, l);
        let rend = find_token_end(right, r);

        if left[l] == '[' as u8 || right[r] == '[' as u8 {
            if let Some(result) = is_in_order(&left[lstart..lend], &right[rstart..rend]) {
                return Some(result);
            }
        } else {
            let diff = String::from_utf8_lossy(&right[rstart..rend]).parse::<i32>().unwrap()
                - String::from_utf8_lossy(&left[lstart..lend]).parse::<i32>().unwrap();

            if diff != 0 {
                return Some(diff > 0);
            }
        }

        l = lend + 1;
        r = rend + 1;
    }

    if l >= left.len() && r >= right.len() { None } else { Some(l >= left.len()) }
}

fn part1(rows: &Vec<String>) -> usize {
    let mut sum: usize = 0;

    for (idx, (left, right)) in rows.iter().step_by(3).zip(
        rows.iter().skip(1).step_by(3)).enumerate() {
        if let Some(true) = is_in_order(left.as_bytes(), right.as_bytes()) {
            sum += idx + 1
        }
    }

    sum
}

fn part2(rows: &Vec<String>) -> usize {
    let mut relevant = vec!["[[2]]".as_bytes(), "[[6]]".as_bytes()];
    relevant.extend(rows.iter()
        .filter(|&row| !row.trim().is_empty())
        .map(|row| row.as_bytes()));

    relevant.sort_by(|&a, &b| match is_in_order(a, b) {
        Some(result) => if result { Ordering::Less } else { Ordering::Greater },
        None => Ordering::Equal
    });

    relevant.iter().enumerate()
        .filter(|(_, &value)| value == "[[2]]".as_bytes() || value == "[[6]]".as_bytes())
        .map(|(idx, _)| idx + 1)
        .fold(1, |a, b| a * b)
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
        assert_eq!(part2(&lines), 140);
    }
}
