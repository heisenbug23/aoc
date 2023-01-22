use std::cmp::{max, min};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;

fn readlines(filename: &str) -> Vec<String> {
    let file = BufReader::new(File::open(&filename)
        .expect(format!("File \"{filename}\" not found").as_ref()));
    return file.lines().map(Result::unwrap).collect();
}

fn parse(rows: &Vec<String>) -> Vec<((i32, i32), (i32, i32))> {
    rows.iter()
        .map(|row| row.split(' ').collect::<Vec<_>>())
        .map(|tokens| (
            (
                tokens[2].split('=').collect::<Vec<_>>()[1].trim_matches(',').parse().unwrap(),
                tokens[3].split('=').collect::<Vec<_>>()[1].trim_matches(':').parse().unwrap()
            ), (
                tokens[8].split('=').collect::<Vec<_>>()[1].trim_matches(',').parse().unwrap(),
                tokens[9].split('=').collect::<Vec<_>>()[1].trim().parse().unwrap()
            )
        )).collect()
}

fn distance(a: &(i32, i32), b: &(i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn merge_ranges(ranges: &Vec<RangeInclusive<i32>>) -> Vec<RangeInclusive<i32>> {
    let mut merged_ranges: Vec<RangeInclusive<i32>> = vec!();
    merged_ranges.reserve(ranges.len());
    let mut changed = false;

    'range_loop: for range in ranges {
        for merged_range in &mut merged_ranges {
            if range.start() <= merged_range.start() && range.end() >= merged_range.start() ||
                merged_range.start() <= range.start() && merged_range.end() >= range.start() {

                *merged_range = min(*range.start(), *merged_range.start())..=max(*range.end(), *merged_range.end());
                changed = true;
                continue 'range_loop;
            }
        }
        merged_ranges.push(range.clone());
    }

    assert_eq!(merged_ranges.len() <= ranges.len(), true);
    if changed { merge_ranges(&merged_ranges) } else { merged_ranges }
}

fn find_excluded_ranges(sensors_with_beacons: &Vec<((i32, i32), (i32, i32))>, row_of_interest: i32) -> Vec<RangeInclusive<i32>> {
    let mut excluded_ranges: Vec<RangeInclusive<i32>> = vec!();
    excluded_ranges.reserve(sensors_with_beacons.len());

    for sensor_with_beacon in sensors_with_beacons {
        let dist_to_beacon = distance(&sensor_with_beacon.0, &sensor_with_beacon.1);
        let dist_to_row_of_interest = distance(&sensor_with_beacon.0, &(sensor_with_beacon.0.0, row_of_interest));

        if dist_to_row_of_interest <= dist_to_beacon {
            let x_range = dist_to_beacon - dist_to_row_of_interest;
            excluded_ranges.push(sensor_with_beacon.0.0 - x_range..=sensor_with_beacon.0.0 + x_range);
        }
    }

    merge_ranges(&excluded_ranges)
}

fn part1(rows: &Vec<String>, row_of_interest: i32) -> i32 {
    let sensors_with_beacons = parse(rows);
    let ranges = find_excluded_ranges(&sensors_with_beacons, row_of_interest);
    let mut beacons: Vec<i32> = sensors_with_beacons.iter().filter(|b| b.1.1 == row_of_interest).map(|s| s.1.0).collect();
    beacons.sort_unstable();
    beacons.dedup();

    ranges.iter()
        .map(|range| range.end() - range.start() + 1)
        .sum::<i32>() - beacons.len() as i32
}

fn part2(rows: &Vec<String>, searchspace: ((i32, i32), (i32, i32))) -> i64 {
    let sensors_with_beacons = parse(rows);

    for y in searchspace.1.0..=searchspace.1.1 {
        let ranges = find_excluded_ranges(&sensors_with_beacons, y);
        if ranges.len() > 1 {
            return (*ranges[0].end() as i64 + 1) * 4000000 + y as i64;
        }
    }

    panic!("Didn't find a solution");
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let filename = if args.len() == 2 { &args[1] } else { "input" };
    let lines = readlines(filename);
    println!("Part 1: {}", part1(&lines, 2000000));
    println!("Part 2: {}", part2(&lines, ((0, 4000000), (0, 4000000))));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let lines = readlines("test.in");
        assert_eq!(part1(&lines, 10), 26);
        assert_eq!(part2(&lines, ((0, 20), (0, 20))), 56000011);
    }
}
