use std::collections::HashMap;

use advent2021::common;

const DAY: &str = "day10";

fn part1(lines: &[String]) -> usize {
    let mut s_map: HashMap<char, i32> = HashMap::new();
    s_map.insert(')', 3);
    s_map.insert(']', 57);
    s_map.insert('}', 1197);
    s_map.insert('>', 25137);

    lines
        .iter()
        .filter_map(|l| {
            let mut stack = Vec::new();
            for c in l.chars() {
                match c {
                    '(' | '{' | '[' | '<' => {
                        stack.push(c);
                        continue;
                    }
                    ')' | '}' | ']' | '>' => {
                        let val = stack.pop().unwrap();
                        if (val == '<' && c == '>')
                            || (val == '(' && c == ')')
                            || (val == '[' && c == ']')
                            || (val == '{' && c == '}')
                        {
                            continue;
                        }
                        return Some(c);
                    }
                    _ => continue,
                }
            }
            None
        })
        .fold(0, |acc, c| acc + s_map[&c] as usize)
}

fn part2(l: &[String]) -> usize {
    0
}

fn main() {
    common::time_func(|| {
        let lines = common::read_input::<String>(DAY, false);

        println!("Part01: {}", part1(&lines));
        println!("Part02: {}", part2(&lines));
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let lines = common::read_input::<String>(DAY, true);
        assert_eq!(part1(&lines), 26397);
    }

    #[test]
    #[ignore]
    fn test_part2() {
        let lines = common::read_input::<String>(DAY, true);
        assert_eq!(part2(&lines), 5);
    }
}
