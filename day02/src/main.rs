use std::fs::File;
use std::io::{BufRead, BufReader};

fn readlines(filename: &str) -> Vec<String> {
    let file = BufReader::new(File::open(&filename)
        .expect(format!("File \"{filename}\" not found").as_ref()));
    return file.lines().map(Result::unwrap).collect();
}

fn conv(a: &str) -> char {
    match a {
        "A" | "X" => 'R',
        "B" | "Y" => 'P',
        "C" | "Z" => 'S',
        _ => panic!("Unknown input {a}")
    }
}

fn round_winner(a: char, b: char) -> i32 {
    if a == b {
        return 0;
    }
    match (a, b) {
        ('R', 'P') | ('P', 'S') | ('S', 'R') => 1,
        _ => -1
    }
}

fn shape_points(a: char) -> i32 {
    match a {
        'R' => 1,
        'P' => 2,
        'S' => 3,
        _ => panic!("Unknown input {a}")
    }
}

fn outcome_points(a: i32) -> i32 {
    (a + 1) * 3
}

fn part1(rows: &Vec<String>) -> i32 {
    let mut score = 0;
    for row in rows {
        let input = row.split(' ').collect::<Vec<&str>>();
        let opp = conv(input[0]);
        let mine = conv(input[1]);
        let winner = round_winner(opp, mine);
        score += shape_points(mine) + outcome_points(winner);
    }
    return score;
}

fn part2(rows: &Vec<String>) -> i32 {
    let mut score = 0;
    for row in rows {
        let input = row.split(' ').collect::<Vec<&str>>();
        let opp = conv(input[0]);
        let expected = match input[1] {
            "X" => -1,
            "Y" => 0,
            "Z" => 1,
            _ => panic!("Unknown input")
        };

        for needed in ['R', 'P', 'S'] {
            if round_winner(opp, needed) == expected {
                score += shape_points(needed) + outcome_points(expected);
                break;
            }
        }
    }

    return score;
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
        assert_eq!(part1(&lines), 15);
        assert_eq!(part2(&lines), 12);
    }
}
