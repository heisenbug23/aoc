use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn readlines(filename: &str) -> Vec<String> {
    let file = BufReader::new(File::open(&filename)
        .expect(format!("File \"{filename}\" not found").as_ref()));
    return file.lines().map(Result::unwrap).collect();
}

fn from_snafu(snafu: &str) -> i64 {
    snafu.as_bytes().iter().rev().enumerate()
        .fold(0, |num, (shift, &fac)| {
            let fac = if fac == b'-' { -1 } else if fac == b'=' { -2 } else { fac as i64 - '0' as i64 };
            num + fac * 5i64.pow(shift as u32)
        })
}

fn to_snafu(mut number: i64) -> String {
    let mut result = String::new();

    let mut div = 1;
    while div * 5 / 2 < number {
        div *= 5;
    }

    while div > 0 {
        let mut n = number / div;
        let remaining_reachable = div / 2;

        if number >= 0 {
            while number - n * div > remaining_reachable {
                n += 1;
            }
        } else {
            while (number - n * div).abs() > remaining_reachable {
                n -= 1;
            }
        }

        assert!(n >= -2 && n <= 2);
        number -= n * div;
        result.push(if n == -1 { '-' } else if n == -2 { '=' } else { ('0' as u8 + n as u8) as char });

        div /= 5;
    }

    result
}

fn part1(rows: &Vec<String>) -> String {
    to_snafu(rows.iter().map(|r| from_snafu(r)).sum())
}

fn part2(_: &Vec<String>) -> usize {
    todo!()
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
    fn from_snafu_works() {
        assert_eq!(from_snafu("0"), 0);
        assert_eq!(from_snafu("1"), 1);
        assert_eq!(from_snafu("2"), 2);
        assert_eq!(from_snafu("1="), 3);
        assert_eq!(from_snafu("1-"), 4);
        assert_eq!(from_snafu("10"), 5);
        assert_eq!(from_snafu("11"), 6);
        assert_eq!(from_snafu("12"), 7);
        assert_eq!(from_snafu("2="), 8);
        assert_eq!(from_snafu("2-"), 9);
        assert_eq!(from_snafu("20"), 10);
        assert_eq!(from_snafu("1=0"), 15);
        assert_eq!(from_snafu("1-0"), 20);
        assert_eq!(from_snafu("1=11-2"), 2022);
        assert_eq!(from_snafu("1-0---0"), 12345);
        assert_eq!(from_snafu("1121-1110-1=0"), 314159265);
    }

    #[test]
    fn to_snafu_works() {
        assert_eq!(to_snafu(0), "0");
        assert_eq!(to_snafu(1), "1");
        assert_eq!(to_snafu(2), "2");
        assert_eq!(to_snafu(3), "1=");
        assert_eq!(to_snafu(4), "1-");
        assert_eq!(to_snafu(5), "10");
        assert_eq!(to_snafu(6), "11");
        assert_eq!(to_snafu(7), "12");
        assert_eq!(to_snafu(8), "2=");
        assert_eq!(to_snafu(9), "2-");
        assert_eq!(to_snafu(10), "20");
        assert_eq!(to_snafu(15), "1=0");
        assert_eq!(to_snafu(20), "1-0");
        assert_eq!(to_snafu(2022), "1=11-2");
        assert_eq!(to_snafu(12345), "1-0---0");
        assert_eq!(to_snafu(314159265), "1121-1110-1=0");
    }

    #[test]
    fn it_works() {
        let lines = readlines("test.in");
        assert_eq!(part1(&lines), "2=-1=0");
        // assert_eq!(part2(&lines), );
    }
}
