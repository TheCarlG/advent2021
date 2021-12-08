use std::collections::HashMap;

use advent2021::common;

fn part1(input: Vec<(Vec<&str>, Vec<&str>)>) -> i32 {
    let mut m: HashMap<usize, i32> = HashMap::new();

    input.iter().for_each(|v| {
        v.1.iter().for_each(|k| {
            let k = (**k).len();
            m.entry(k).and_modify(|val| *val += 1).or_insert(1);
        })
    });
    m[&2] + m[&3] + m[&4] + m[&7]
}

fn part2(input: Vec<(Vec<&str>, Vec<&str>)>) -> i32 {
    0
}
fn main() {
    common::time_func(|| {
        let input = common::read_input::<String>("input/day08.data");
        let input: Vec<(Vec<&str>, Vec<&str>)> = input
            .iter()
            .map(|r| {
                let t = r.split_once('|').unwrap();
                let p1: Vec<&str> = t.0.trim().split(' ').collect();
                let p2: Vec<&str> = t.1.trim().split(' ').collect();

                (p1, p2)
            })
            .collect();

        println!("Part01: {}", part1(input.clone()));
        println!("Part02: {}", part2(input));
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = common::read_input::<String>("input/day08.test");
        let input: Vec<(Vec<&str>, Vec<&str>)> = input
            .iter()
            .map(|r| {
                let t = r.split_once('|').unwrap();
                let p1: Vec<&str> = t.0.trim().split(' ').collect();
                let p2: Vec<&str> = t.1.trim().split(' ').collect();

                (p1, p2)
            })
            .collect();

        assert_eq!(part1(input), 26);
    }

    #[test]
    #[ignore]
    fn test_part2() {
        let input = common::read_input::<String>("input/day08.test");
        let input: Vec<(Vec<&str>, Vec<&str>)> = input
            .iter()
            .map(|r| {
                let t = r.split_once('|').unwrap();
                let p1: Vec<&str> = t.0.trim().split(' ').collect();
                let p2: Vec<&str> = t.1.trim().split(' ').collect();

                (p1, p2)
            })
            .collect();

        assert_eq!(part2(input), 61229);
    }
}
