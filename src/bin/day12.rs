use advent2021::common;
use core::fmt;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

const DAY: &str = "day12";

struct Map {
    nodes: HashMap<i32, Node>,
}

impl Map {
    fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    fn find_path_to(
        &self,
        start: i32,
        stop: i32,
        visited: &mut HashSet<i32>,
        path: &mut Vec<i32>,
    ) -> i32 {
        if start == stop {
            return 1;
        }

        let n = &self.nodes[&start];
        if !n.big {
            visited.insert(start);
        }

        let mut num = 0;

        for cc in &n.adjecent {
            if !visited.contains(cc) {
                path.push(*cc);
                num += self.find_path_to(*cc, stop, visited, path);
            }
        }
        visited.remove(&start);

        num
    }
}

#[derive(Debug)]
struct Node {
    name: String,
    big: bool,
    adjecent: Vec<i32>,
}

impl Node {
    fn new(name: String, big: bool) -> Self {
        Node {
            name,
            big,
            adjecent: Vec::new(),
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn part1(lines: &[String]) -> i32 {
    let mut map = Map::new();
    let mut n_map: HashMap<&str, i32> = HashMap::new();

    lines.iter().for_each(|l| {
        let parts = l.split_once('-').unwrap();
        let p = if n_map.contains_key(&parts.0) {
            n_map[&parts.0]
        } else {
            let idx = n_map.len() as i32;
            n_map.insert(parts.0, idx);
            idx
        };

        let c = if n_map.contains_key(&parts.1) {
            n_map[&parts.1]
        } else {
            let idx = n_map.len() as i32;
            n_map.insert(parts.1, idx);
            idx
        };

        map.nodes.entry(p).or_insert_with(|| {
            let big = parts.0 != parts.0.to_lowercase();
            Node::new(parts.0.to_string(), big)
        });

        map.nodes.entry(c).or_insert_with(|| {
            let big = parts.1 != parts.1.to_lowercase();
            Node::new(parts.1.to_string(), big)
        });

        let parent = map.nodes.get_mut(&p).unwrap();
        parent.adjecent.push(c);

        let adj = map.nodes.get_mut(&c).unwrap();
        adj.adjecent.push(p);
    });

    let start = n_map["start"];
    let stop = n_map["end"];

    let s = &map.nodes[&start];

    let mut num = 0;
    for adj in &s.adjecent {
        let mut visited: HashSet<i32> = HashSet::new();
        visited.insert(start);
        let mut path: Vec<i32> = vec![start];
        num += map.find_path_to(*adj, stop, &mut visited, &mut path);
    }

    num
}

fn part2(_lines: &[String]) -> i32 {
    0
}

fn main() {
    common::time_func(|| {
        let lines = common::read_input::<String>(DAY, false);

        println!("Part 1: {}", part1(&lines));
        println!("Part 2: {}", part2(&lines));
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let lines = common::read_input::<String>(DAY, true);
        assert_eq!(part1(&lines), 10);
    }
    #[test]
    fn test_part1_2() {
        let lines = common::read_input::<String>("day12-2", true);
        assert_eq!(part1(&lines), 19);
    }
    #[test]
    fn test_part1_3() {
        let lines = common::read_input::<String>("day12-3", true);
        assert_eq!(part1(&lines), 226);
    }

    #[test]
    #[ignore]
    fn test_part2() {
        let lines = common::read_input::<String>(DAY, true);
        assert_eq!(part2(&lines), 5);
    }
}
