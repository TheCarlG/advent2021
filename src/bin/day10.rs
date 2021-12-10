use std::collections::HashMap;

use advent2021::common;

const DAY: &str = "day10";

fn part1(lines: &[String]) -> usize {
    let s_map: HashMap<char, i32> = [(')', 3), (']', 57), ('}', 1197), ('>', 25137)]
        .iter()
        .cloned()
        .collect();

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

fn part2(lines: &[String]) -> u128 {
    let s_map: HashMap<char, i32> = [(')', 1), (']', 2), ('}', 3), ('>', 4)]
        .iter()
        .cloned()
        .collect();

    let pair: HashMap<char, char> = [('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]
        .iter()
        .cloned()
        .collect();

    let mut res: Vec<u128> = lines
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
                        return None;
                    }
                    _ => continue,
                }
            }

            Some(
                stack
                    .iter()
                    .rev()
                    .map(|c| pair[c])
                    .fold(0_u128, |acc, c| (acc * 5) + s_map[&c] as u128),
            )
        })
        .collect();

    res.sort_unstable();

    res[res.len() / 2]
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
    fn test_part2() {
        let lines = common::read_input::<String>(DAY, true);
        assert_eq!(part2(&lines), 288957);
    }
}
