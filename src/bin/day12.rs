use advent2021::common;
use core::fmt;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

const DAY: &str = "day12";

struct Map<'a> {
    nodes: HashMap<&'a str, Node<'a>>,
}

impl<'a> Map<'a> {
    fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    fn read_lines(&mut self, lines: &'a [String]) {
        lines.iter().for_each(|l| {
            let (p, c) = l.split_once('-').unwrap();

            self.nodes.entry(p).or_insert_with(|| {
                let big = p != p.to_lowercase();
                Node::new(p.to_string(), big)
            });

            self.nodes.entry(c).or_insert_with(|| {
                let big = c != c.to_lowercase();
                Node::new(c.to_string(), big)
            });

            let parent = self.nodes.get_mut(&p).unwrap();
            parent.adjecent.push(c);

            let adj = self.nodes.get_mut(&c).unwrap();
            adj.adjecent.push(p);
        });
    }

    fn find_path_to(&self, start: &'a str, stop: &'a str, visited: &mut HashSet<&'a str>) -> i32 {
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
                num += self.find_path_to(*cc, stop, visited);
            }
        }
        visited.remove(&start);

        num
    }

    fn find_path_to2(
        &self,
        start: &'a str,
        stop: &'a str,
        visited: &mut HashMap<&'a str, i32>,
        ext_search: &mut bool,
    ) -> i32 {
        if start == stop {
            return 1;
        }
        let n = &self.nodes[&start];

        let val = visited.entry(start).or_insert(0);
        if !n.big {
            *val += 1;
            if *val >= 2 && *ext_search {
                *ext_search = false;
            }
        }

        let mut num = 0;

        for adj in &n.adjecent {
            let visits = *visited.get(adj).unwrap_or(&0);
            if *adj != "start" && (visits < 1 || (visits < 2 && *ext_search)) {
                num += self.find_path_to2(*adj, stop, visited, ext_search);
            }
        }

        let val = visited.get_mut(start).unwrap();
        if *val > 0 {
            if *val == 2 {
                *ext_search = true;
            }
            *val -= 1;
        }

        num
    }
}

#[derive(Debug)]
struct Node<'a> {
    name: String,
    big: bool,
    adjecent: Vec<&'a str>,
}

impl<'a> Node<'a> {
    fn new(name: String, big: bool) -> Self {
        Node {
            name,
            big,
            adjecent: Vec::new(),
        }
    }
}

impl<'a> Display for Node<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn part1(lines: &[String]) -> i32 {
    let mut map = Map::new();

    map.read_lines(lines);

    let s = &map.nodes["start"];

    let mut num = 0;
    for _adj in &s.adjecent {}
    let mut visited: HashSet<&str> = HashSet::new();
    visited.insert("start");
    num += map.find_path_to("start", "end", &mut visited);

    num
}

fn part2(lines: &[String]) -> i32 {
    let mut map = Map::new();

    map.read_lines(lines);

    let mut num = 0;
    let mut visited: HashMap<&str, i32> = HashMap::new();
    num += map.find_path_to2("start", "end", &mut visited, &mut true);

    num
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
    fn test_part2() {
        let lines = common::read_input::<String>(DAY, true);
        assert_eq!(part2(&lines), 36);
    }

    #[test]
    fn test_part2_2() {
        let lines = common::read_input::<String>("day12-2", true);
        assert_eq!(part2(&lines), 103);
    }

    #[test]
    fn test_part2_3() {
        let lines = common::read_input::<String>("day12-3", true);
        assert_eq!(part2(&lines), 3509);
    }
}
