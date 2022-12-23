use std::cmp::{max, min};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

fn readlines(filename: &str) -> Vec<String> {
    let file = BufReader::new(File::open(&filename)
        .expect(format!("File \"{filename}\" not found").as_ref()));
    return file.lines().map(Result::unwrap).collect();
}

#[derive(Debug)]
struct Valve {
    name: String,
    flow_rate: u64,
    tunnels: Vec<String>,
}

fn parse(rows: &Vec<String>) -> Vec<Valve> {
    rows.iter()
        .map(|row| row.split(' ').collect_vec())
        .map(|tokens| Valve {
            name: tokens[1].to_string(),
            flow_rate: tokens[4].split('=').collect_vec()[1].trim_matches(';').parse().unwrap(),
            tunnels: tokens[9..].iter().map(|t| t.trim_matches(',').to_string()).collect(),
        }).collect()
}

// implements Floyd–Warshall algorithm (O(n^3) but simple to code)
fn find_distances(valves: &Vec<Valve>) -> Vec<Vec<u64>> {
    let index_by_name: HashMap<String, usize> = valves.iter().enumerate()
        .map(|(idx, v)| (v.name.clone(), idx))
        .collect();
    let mut distances = vec![vec![u64::MAX; valves.len()]; valves.len()];

    (0..valves.len()).for_each(|i| distances[i][i] = 0);
    valves.iter().for_each(|valve| valve.tunnels.iter().for_each(|t|
        distances[*index_by_name.get(&valve.name).unwrap()][*index_by_name.get(t).unwrap()] = 1));

    for k in 0..valves.len() {
        for i in 0..valves.len() {
            for j in 0..valves.len() {
                if distances[i][k] != u64::MAX && distances[j][k] != u64::MAX {
                    distances[i][j] = min(distances[i][j], distances[i][k] + distances[j][k]);
                }
            }
        }
    }

    distances
}

fn find_shortest_cached_path(valves: &Vec<Valve>,
                             distances: &Vec<Vec<u64>>,
                             cache: &mut HashMap<(u64, usize, usize), u64>,
                             initial_remaining_minutes: u64,
                             start_idx: usize,
                             to_visit: &[usize]) -> u64 {
    if initial_remaining_minutes == 0 {
        return 0;
    }

    let reachable = to_visit.iter()
        // + 1 for opening the valve and + 1 for at least one minute left after opening the valve
        .filter(|&&i| distances[start_idx][i] + 2 <= initial_remaining_minutes)
        .cloned()
        .collect_vec();

    let cache_key = (initial_remaining_minutes, start_idx, reachable.iter()
        .fold(0, |bits, idx| bits | (1 << idx)));
    if let Some(&cached) = cache.get(&cache_key) {
        return cached;
    }

    let mut best = valves[start_idx].flow_rate * (initial_remaining_minutes - 1);

    for &dest_idx in &reachable {
        let remaining_to_visit = reachable.clone().into_iter()
            .filter(|&i| i != dest_idx)
            .collect_vec();
        let mut remaining_minutes = initial_remaining_minutes;
        let mut score = 0;

        if valves[start_idx].flow_rate > 0 {
            remaining_minutes -= 1;
            score += valves[start_idx].flow_rate * remaining_minutes;
        };

        let dist = distances[start_idx][dest_idx];
        if dist + 2 < remaining_minutes {
            remaining_minutes -= dist;

            score += find_shortest_cached_path(valves,
                                               distances,
                                               cache,
                                               remaining_minutes,
                                               dest_idx,
                                               &remaining_to_visit);
        }

        best = max(best, score);
    }

    assert_ne!(cache.contains_key(&cache_key), true);
    cache.insert(cache_key, best);

    best
}

fn part1(rows: &Vec<String>) -> u64 {
    let valves = parse(rows);
    let distances = find_distances(&valves);
    let start_idx = valves.iter().enumerate()
        .find(|(_, v)| v.name == "AA")
        .unwrap().0;
    let to_visit = valves.iter().enumerate()
        .filter(|(_, v)| v.flow_rate > 0)
        .map(|(idx, _)| idx)
        .collect_vec();
    let mut cache: HashMap<(u64, usize, usize), u64> = HashMap::new();

    find_shortest_cached_path(&valves,
                              &distances,
                              &mut cache,
                              30,
                              start_idx,
                              &to_visit)
}

fn part2(rows: &Vec<String>) -> u64 {
    let valves = parse(rows);
    let distances = find_distances(&valves);
    let start_idx = valves.iter().enumerate()
        .find(|(_, v)| v.name == "AA")
        .unwrap().0;
    let to_visit = valves.iter().enumerate()
        .filter(|(_, v)| v.flow_rate > 0)
        .map(|(idx, _)| idx)
        .collect_vec();
    let mut cache: HashMap<(u64, usize, usize), u64> = HashMap::new();
    let mut best = 0;

    for route_a in to_visit.iter().cloned().powerset() {
        if route_a.len() > to_visit.len() / 2 {
            continue;
        }
        let route_b = to_visit.iter()
            .filter(|i| !route_a.contains(i))
            .cloned()
            .collect_vec();
        best = max(best,
                   [route_a, route_b].iter()
                       .map(|route| find_shortest_cached_path(&valves,
                                                              &distances,
                                                              &mut cache,
                                                              26,
                                                              start_idx,
                                                              route))
                       .sum());
    }

    best
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
        assert_eq!(part1(&lines), 1651);
        assert_eq!(part2(&lines), 1707);
    }
}
