use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn readlines(filename: &str) -> Vec<String> {
    let file = BufReader::new(File::open(&filename)
        .expect(format!("File \"{filename}\" not found").as_ref()));
    return file.lines().map(Result::unwrap).collect();
}

struct Grid {
    heightmap: Vec<Vec<u8>>,
    start: (usize, usize),
    end: (usize, usize),
}

fn parse(rows: &Vec<String>) -> Grid {
    let mut grid = Grid {
        heightmap: vec!(),
        start: (0, 0),
        end: (0, 0),
    };

    for (ridx, row) in rows.iter().enumerate() {
        let mut heights: Vec<u8> = vec!();
        for (cidx, height) in row.bytes().enumerate() {
            if height == 'S' as u8 {
                grid.start = (ridx, cidx);
                heights.push(0);
            } else if height == 'E' as u8 {
                grid.end = (ridx, cidx);
                heights.push('z' as u8 - 'a' as u8);
            } else {
                heights.push(height as u8 - 'a' as u8);
            }
        }
        grid.heightmap.push(heights);
    }

    grid
}

#[derive(Clone)]
struct Node {
    distance: usize,
    predecessor: (usize, usize),
    visited: bool,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct HeapNode {
    pos: (usize, usize),
    distance: usize,
}

impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // other vs self to get the node with the smallest distance from the heap
        other.distance.cmp(&self.distance)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// implements Dijkstra's algorithm
fn find_shortest_path(grid: &Grid) -> Option<Vec<(usize, usize)>> {
    let mut nodes: Vec<Vec<Node>> = grid.heightmap.iter()
        .map(|row| vec![Node {
            distance: usize::MAX,
            predecessor: (usize::MAX, usize::MAX),
            visited: false,
        }; row.len()])
        .collect();

    let mut heap: BinaryHeap<HeapNode> = BinaryHeap::new();

    nodes[grid.start.0][grid.start.1].distance = 0;
    heap.push(HeapNode {
        pos: grid.start,
        distance: 0,
    });

    while let Some(node) = heap.pop() {
        // we add nodes more than once, maybe we've already been here
        if nodes[node.pos.0][node.pos.1].visited {
            continue;
        }

        nodes[node.pos.0][node.pos.1].visited = true;

        if node.pos == grid.end {
            break;
        }

        for dir in [(-1i8, 0i8), (0, -1), (0, 1), (1, 0)] {
            if dir.0 < 0 && node.pos.0 == 0 || dir.1 < 0 && node.pos.1 == 0 {
                continue;
            }

            let new_pos = (
                if dir.0 < 0 { node.pos.0 - (dir.0.abs() as usize) } else { node.pos.0 + dir.0 as usize },
                if dir.1 < 0 { node.pos.1 - (dir.1.abs() as usize) } else { node.pos.1 + dir.1 as usize }
            );

            if new_pos.0 == nodes.len() || new_pos.1 == nodes[new_pos.0].len()
                || nodes[new_pos.0][new_pos.1].visited
                || nodes[node.pos.0][node.pos.1].distance + 1 >= nodes[new_pos.0][new_pos.1].distance
                || grid.heightmap[new_pos.0][new_pos.1] as i8 - grid.heightmap[node.pos.0][node.pos.1] as i8 > 1 {
                continue;
            }

            nodes[new_pos.0][new_pos.1].distance = nodes[node.pos.0][node.pos.1].distance + 1;
            nodes[new_pos.0][new_pos.1].predecessor = node.pos;

            heap.push(HeapNode {
                pos: new_pos,
                distance: nodes[new_pos.0][new_pos.1].distance,
            });
        }
    }

    if !nodes[grid.end.0][grid.end.1].visited {
        return None;
    }

    let mut path: Vec<(usize, usize)> = vec!();
    let mut pos = grid.end;
    while pos != grid.start {
        path.push(pos);
        pos = nodes[pos.0][pos.1].predecessor;
    }
    path.push(grid.start);

    path.reverse();
    Some(path)
}

fn part1(rows: &Vec<String>) -> usize {
    let grid = parse(rows);
    find_shortest_path(&grid).unwrap().len() - 1
}

fn part2(rows: &Vec<String>) -> usize {
    let mut grid = parse(rows);
    let starting_points: Vec<(usize, usize)> = grid.heightmap.iter().enumerate()
        .map(|(ridx, row)| row.iter().enumerate()
            .filter(|(_, &height)| height == 0)
            .map(|(cidx, _)| (ridx, cidx))
            .collect::<Vec<(usize, usize)>>())
        .flatten()
        .collect();
    starting_points.iter().map(|start| {
        grid.start = *start;
        *find_shortest_path(&grid).map(|result| result.len() - 1).get_or_insert(usize::MAX)
    }).min().unwrap()
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
        assert_eq!(part1(&lines), 31);
        assert_eq!(part2(&lines), 29);
    }
}
