use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::sh::{Command, ListingEntry, parse_listing};

fn readlines(filename: &str) -> Vec<String> {
    let file = BufReader::new(File::open(&filename)
        .expect(format!("File \"{filename}\" not found").as_ref()));
    return file.lines().map(Result::unwrap).collect();
}

mod fs {
    pub struct File {
        pub name: String,
        pub size: u64,
    }

    pub struct Dir {
        pub name: String,
        pub dirs: Vec<Dir>,
        pub files: Vec<File>,
    }

    impl Dir {
        pub fn new(n: &str) -> Self {
            Self {
                name: n.to_string(),
                dirs: vec!(),
                files: vec!(),
            }
        }

        pub fn add_dir(&mut self, name: &str) -> &mut Dir {
            self.dirs.push(Dir::new(name));
            return self.dirs.last_mut().unwrap();
        }

        pub fn add_file(&mut self, name: &str, size: u64) -> &mut File {
            self.files.push(File { name: name.to_string(), size });
            return self.files.last_mut().unwrap();
        }

        pub fn size(&self) -> u64 {
            self.files.iter().map(|f| f.size).sum::<u64>()
                + self.dirs.iter().map(|d| d.size()).sum::<u64>()
        }
    }
}

mod sh {
    pub enum Command {
        CD { dest: String },
        LS,
        UNKNOWN { line: String },
    }

    pub enum ListingEntry {
        Command(Command),
        Output(String),
    }

    pub fn parse_listing_line(line: &str) -> ListingEntry {
        let token: Vec<&str> = line.split(' ').collect();
        if token[0] == "$" {
            return ListingEntry::Command(match token[1] {
                "cd" => Command::CD { dest: token[2].to_string() },
                "ls" => Command::LS,
                _ => Command::UNKNOWN { line: line.to_string() }
            });
        }
        return ListingEntry::Output(line.to_string());
    }

    pub fn parse_listing(lines: &Vec<String>) -> Vec<ListingEntry> {
        lines.iter().map(|line| parse_listing_line(line)).collect()
    }
}


fn process_terminal_output(rows: &Vec<String>) -> fs::Dir {
    let mut root = fs::Dir::new("/");
    let mut current_dir = &mut root;
    let mut last_cmd = "";
    // TODO it would be easier to remember the parent of a directory and walk one node back;
    // somehow I didn't get the lifetime of the back reference working
    let mut visited_dirs: Vec<String> = vec!(current_dir.name.clone());

    for listing_entry in parse_listing(rows) {
        match listing_entry {
            ListingEntry::Command(cmd) => {
                match cmd {
                    Command::CD { dest } => {
                        last_cmd = "cd";
                        current_dir = match dest.as_str() {
                            ".." => {
                                visited_dirs.pop();
                                let prev_dir = visited_dirs.iter().skip(1).fold(
                                    &mut root,
                                    |dir, to_visit| dir.dirs.iter_mut().find(|dir| dir.name == *to_visit).unwrap(),
                                );
                                // drop the prev dir as it will be added again later on as the current one
                                visited_dirs.pop();
                                prev_dir
                            }
                            "/" => {
                                visited_dirs.clear();
                                &mut root
                            }
                            dest => match current_dir.dirs.iter().position(|dir| dir.name == dest) {
                                Some(pos) => &mut current_dir.dirs[pos],
                                None => current_dir.add_dir(dest)
                            }
                        };
                        visited_dirs.push(current_dir.name.clone());
                    }
                    Command::LS => last_cmd = "ls",
                    Command::UNKNOWN { line } => panic!("{}", format!("Unknown command found: {}", line))
                }
            }
            ListingEntry::Output(line) => {
                if last_cmd == "ls" {
                    let token: Vec<&str> = line.split(' ').collect();
                    if token[0] == "dir" {
                        current_dir.add_dir(token[1]);
                    } else {
                        current_dir.add_file(token[1], token[0].parse::<u64>().unwrap());
                    }
                } else {
                    panic!("Unexpected output in listing for mode {last_cmd}");
                }
            }
        }
    };

    return root;
}

#[allow(dead_code)]
fn print_dirs(dir: &fs::Dir, indent: &str) {
    println!("{}{}: {}", indent, dir.name, dir.size());
    for dir in dir.dirs.iter() {
        print_dirs(dir, (" ".to_string() + indent).as_str())
    }
}

fn find_dirs<'d, P>(dir: &'d fs::Dir, predicate: &'d P) -> Vec<&'d fs::Dir>
    where P: Fn(&fs::Dir) -> bool
{
    let mut dirs = vec!();

    if predicate(dir) {
        dirs.push(dir);
    }

    for dir in dir.dirs.iter() {
        dirs.extend(find_dirs(dir, predicate));
    }

    return dirs;
}

fn part1(rows: &Vec<String>) -> u64 {
    let root = process_terminal_output(rows);
    let dirs = find_dirs(&root, &|dir| dir.size() < 100_000);
    dirs.iter().map(|dir| dir.size()).sum()
}

fn part2(rows: &Vec<String>) -> u64 {
    const ALL_SIZE: u64 = 70000000;
    const FREE_UPDATE_SIZE: u64 = 30000000;

    let root = process_terminal_output(rows);
    let needed_size = FREE_UPDATE_SIZE - (ALL_SIZE - root.size());

    let predicate = |dir: &fs::Dir| dir.size() >= needed_size;
    let dirs = find_dirs(&root, &predicate);

    dirs.iter().map(|dir| dir.size()).min().unwrap()
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
        assert_eq!(part1(&lines), 95437);
        assert_eq!(part2(&lines), 24933642);
    }
}
