use std::cmp::max;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn readlines(filename: &str) -> Vec<String> {
    let file = BufReader::new(File::open(&filename)
        .expect(format!("File \"{filename}\" not found").as_ref()));
    return file.lines().map(Result::unwrap).collect();
}

type Costs = [u32; 3];
type Blueprint = [Costs; 4];

#[derive(Clone)]
struct Inventory {
    num_robots: [u32; 4],
    resources: [u32; 4],
}

impl Inventory {
    fn collect_current_resources(&mut self) -> &mut Self {
        self.resources.iter_mut().zip(self.num_robots.iter())
            .for_each(|(res, &num_robots)| *res += num_robots);
        self
    }
}

fn parse(rows: &Vec<String>) -> Vec<Blueprint> {
    let mut blueprints: Vec<Blueprint> = vec!();

    for row in rows {
        let token = row.split(' ').collect::<Vec<_>>();
        blueprints.push([
            [token[6].parse().unwrap(), 0, 0],
            [token[12].parse().unwrap(), 0, 0],
            [token[18].parse().unwrap(), token[21].parse().unwrap(), 0],
            [token[27].parse().unwrap(), 0, token[30].parse().unwrap()],
        ]);
    }

    blueprints
}

fn find_max_geodes_rek(blueprint: &Blueprint,
                       inventory: &Inventory,
                       remaining_minutes: u32,
                       buildable_over_time: i32,
                       not_to_build: i32) -> u32 {
    let mut best = *inventory.resources.last().unwrap();

    if remaining_minutes == 0 {
        return best;
    }

    let buildable_with_current_resources = blueprint.iter().enumerate()
        .filter(|(_, costs)| costs.iter().enumerate()
            .all(|(res_idx, &costs)| inventory.resources[res_idx] >= costs))
        .map(|(res_idx, _)| res_idx)
        .fold(0, |a, b| a | (1 << b));

    let try_to_build = (0..inventory.num_robots.len())
        .filter(|&res_idx| not_to_build & (1 << res_idx) == 0)
        .filter(|&res_idx| buildable_with_current_resources & (1 << res_idx) != 0);

    // try without buying anything at all
    if buildable_over_time != buildable_with_current_resources {
        best = max(best, find_max_geodes_rek(blueprint,
                                             inventory.clone().collect_current_resources(),
                                             remaining_minutes - 1,
                                             buildable_over_time,
                                             buildable_with_current_resources));
    }

    // TODO Support building more than one robot per minute
    for to_build in try_to_build {
        let mut new_inventory = inventory.clone();

        blueprint[to_build].iter().enumerate()
            .for_each(|(res_idx, &costs)| { new_inventory.resources[res_idx] -= costs; });

        let still_buildable_with_current_resources = blueprint.iter().enumerate()
            .filter(|(_, costs)| costs.iter().enumerate()
                .all(|(res_idx, &costs)| new_inventory.resources[res_idx] >= costs))
            .map(|(res_idx, _)| res_idx)
            .fold(0, |a, b| a | (1 << b));

        new_inventory.collect_current_resources();
        new_inventory.num_robots[to_build] += 1;

        let buildable_over_time= if new_inventory.num_robots[to_build] > 1 {
            buildable_over_time
        } else {
            blueprint.iter().enumerate()
                .filter(|(_, costs)| costs.iter().enumerate()
                    .all(|(res_idx, &costs)| costs == 0 || new_inventory.num_robots[res_idx] > 0))
                .map(|(res_idx, _)| res_idx)
                .fold(0, |a, b| a | (1 << b))
        };

        best = max(best, find_max_geodes_rek(blueprint,
                                             &new_inventory,
                                             remaining_minutes - 1,
                                             buildable_over_time,
                                             still_buildable_with_current_resources));
    }

    best
}

fn find_max_geodes(blueprint: &Blueprint,
                   inventory: &Inventory,
                   remaining_minutes: u32) -> u32 {
    let buildable_over_time = blueprint.iter().enumerate()
        .filter(|(_, costs)| costs.iter().enumerate()
            .all(|(res_idx, &costs)| costs == 0 || inventory.num_robots[res_idx] > 0))
        .map(|(res_idx, _)| res_idx)
        .fold(0, |a, b| a | (1 << b));

    find_max_geodes_rek(blueprint, inventory, remaining_minutes, buildable_over_time, 0)
}

fn part1(rows: &Vec<String>) -> u32 {
    let blueprints = parse(rows);
    let inventory = Inventory {
        num_robots: [1, 0, 0, 0],
        resources: [0; 4],
    };
    let remaining_minutes = 24u32;

    blueprints.iter().enumerate()
        .map(|(idx, blueprint)| find_max_geodes(blueprint, &inventory, remaining_minutes) * (idx as u32 + 1))
        .sum()
}

fn part2(rows: &Vec<String>) -> u32 {
    let blueprints = parse(rows);
    let inventory = Inventory {
        num_robots: [1, 0, 0, 0],
        resources: [0; 4],
    };
    let remaining_minutes = 32u32;

    blueprints.iter().take(3).enumerate()
        .map(|(idx, blueprint)| find_max_geodes(blueprint, &inventory, remaining_minutes))
        .product()
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
        assert_eq!(part1(&lines), 33);
        assert_eq!(part2(&lines), 56*62);
    }
}
