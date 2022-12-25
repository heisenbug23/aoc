use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::Movement::*;
use crate::Shape::*;

fn readlines(filename: &str) -> Vec<String> {
    let file = BufReader::new(File::open(&filename)
        .expect(format!("File \"{filename}\" not found").as_ref()));
    return file.lines().map(Result::unwrap).collect();
}

//                           #  #
//  #                 #      #  .
// ###                #  ##  #  .
//  #   ####  #..#  ###  ##  #  #
#[derive(Debug)]
enum Shape {
    Cross,
    Horizontal,
    HorizontalFiller,
    ReversedL,
    Square,
    Vertical,
    VerticalFiller,
}

impl Shape {
    fn width(&self) -> u64 {
        match *self {
            Vertical | VerticalFiller => 1,
            Square => 2,
            Cross | ReversedL => 3,
            Horizontal => 4,
            HorizontalFiller => u64::MAX,
        }
    }

    fn height(&self) -> u64 {
        match *self {
            Horizontal | HorizontalFiller => 1,
            Square => 2,
            Cross | ReversedL => 3,
            Vertical => 4,
            VerticalFiller => u64::MAX,
        }
    }
}

enum Movement {
    Down,
    Left,
    Right,
    Up,
}

#[derive(Debug)]
struct Hitbox {
    pos: (u64, u64),
    width: u64,
    height: u64,
}

impl Hitbox {
    fn collidates(&self, other: &Self) -> bool {
        ((self.pos.0 <= other.pos.0 && self.pos.0 + self.width > other.pos.0)
            || (other.pos.0 <= self.pos.0 && other.pos.0 + other.width > self.pos.0))
            && ((self.pos.1 <= other.pos.1 && self.pos.1 + self.height > other.pos.1)
            || (other.pos.1 <= self.pos.1 && other.pos.1 + other.height > self.pos.1))
    }
}

#[derive(Debug)]
struct Rock {
    shape: Shape,
    pos: (u64, u64),
}

impl Rock {
    fn translate(&mut self, movement: Movement) -> &mut Self {
        match movement {
            Down => self.pos.1 -= 1,
            Left => self.pos.0 -= 1,
            Right => self.pos.0 += 1,
            Up => self.pos.1 += 1,
        }
        return self;
    }

    fn hitboxes(&self) -> Vec<Hitbox> {
        let pos = &self.pos;
        match self.shape {
            Cross => vec!(Hitbox { pos: (pos.0, pos.1 + 1), width: (self.shape.width() - 1) / 2, height: 1 },
                          Hitbox { pos: (pos.0 + (self.shape.width() - 1) / 2, pos.1), width: 1, height: self.shape.height() },
                          Hitbox { pos: (pos.0 + (self.shape.width() - 1), pos.1 + 1), width: (self.shape.width() - 1) / 2, height: 1 }),
            ReversedL => vec!(Hitbox { pos: (pos.0, pos.1), width: self.shape.width() - 1, height: 1 },
                              Hitbox { pos: (pos.0 + self.shape.width() - 1, pos.1), width: 1, height: self.shape.height() }),
            _ => vec!(Hitbox { pos: (pos.0, pos.1), width: self.shape.width(), height: self.shape.height() })
        }
    }
}

fn nth_falling_in_sequence(nth: u64) -> Shape {
    match (nth - 1) % 5 {
        0 => Horizontal,
        1 => Cross,
        2 => ReversedL,
        3 => Vertical,
        4 => Square,
        _ => panic!()
    }
}

fn top(rocks: &[Hitbox]) -> Option<u64> {
    rocks.iter()
        .filter(|r| r.height != u64::MAX)
        .map(|r| r.pos.1 + r.height - 1)
        .max()
}

fn find_pattern(data: &[u64], min_length: usize) -> Option<&[u64]> {
    for k in (min_length..data.len() / 2).rev() {
        let pattern = &data[data.len() - k..];
        let proof = &data[data.len() - 2 * k..data.len() - k];
        if pattern == proof {
            return Some(pattern);
        }
    }
    None
}

fn initial_hitboxes() -> Vec<Hitbox> {
    let mut hitboxes: Vec<Hitbox> = vec!();

    hitboxes.extend(Rock { shape: HorizontalFiller, pos: (0, 0) }.hitboxes());
    hitboxes.extend(Rock { shape: VerticalFiller, pos: (0, 0) }.hitboxes());
    hitboxes.extend(Rock { shape: VerticalFiller, pos: (8, 0) }.hitboxes());

    hitboxes
}

fn simulate_nth_rock(hitboxes: &Vec<Hitbox>, jets: &[u8], jet_idx: &mut usize, nth_rock: u64) -> Rock {
    let mut rock = Rock {
        shape: nth_falling_in_sequence(nth_rock),
        // 3 instead of 2 because there is a vertical filler at pos 0
        pos: (3, top(&hitboxes).unwrap() + 4),
    };

    let mut moved = true;
    while moved {
        moved = false;

        // push by jet
        rock.translate(if jets[*jet_idx] == b'<' { Left } else { Right });
        if rock.hitboxes().iter().any(|rhb| hitboxes.iter().any(|hb| hb.collidates(&rhb))) {
            rock.translate(if jets[*jet_idx] == b'<' { Right } else { Left });
        }
        *jet_idx = (*jet_idx + 1) % jets.len();

        // fall down
        rock.translate(Down);
        if rock.hitboxes().iter().any(|rhb| hitboxes.iter().any(|hb| hb.collidates(&rhb))) {
            rock.translate(Up);
        } else {
            moved = true;
        }
    }

    rock
}

fn part1(rows: &Vec<String>) -> u64 {
    let jets = rows[0].as_bytes();
    let mut jet_idx = 0usize;
    let mut hitboxes: Vec<Hitbox> = initial_hitboxes();

    for nth_rock in 1..=2022u64 {
        let rock = simulate_nth_rock(&hitboxes, jets, &mut jet_idx, nth_rock);
        hitboxes.extend(rock.hitboxes());
    }

    top(&hitboxes).unwrap()
}

fn part2(rows: &Vec<String>) -> u64 {
    let jets = rows[0].as_bytes();
    let mut jet_idx = 0usize;
    let mut hitboxes: Vec<Hitbox> = initial_hitboxes();
    let mut nth_rock = 0;
    let mut top_diffs: Vec<u64> = vec!();
    let mut last_top = 0u64;

    loop {
        nth_rock += 1;
        let rock = simulate_nth_rock(&hitboxes, jets, &mut jet_idx, nth_rock);

        hitboxes.extend(rock.hitboxes());

        let new_top = top(&hitboxes).unwrap();
        top_diffs.push(new_top - last_top);
        last_top = new_top;

        if nth_rock % 128 == 0 {
            if let Some(pattern) = find_pattern(&top_diffs, 10) {
                let mut top: u64 = top(&hitboxes).unwrap();

                let num_rocks_needed = 1000000000000u64 - nth_rock;
                top += pattern.iter().sum::<u64>() * (num_rocks_needed / pattern.len() as u64);
                top += pattern.iter().take(num_rocks_needed as usize % pattern.len()).sum::<u64>();

                return top;
            }
        }
    }
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
        assert_eq!(part1(&lines), 3068);
        assert_eq!(part2(&lines), 1514285714288);
    }
}
