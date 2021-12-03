use advent2021::common;
use std::str::FromStr;
use std::string::ParseError;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Forward,
}

struct Command {
    direction: Direction,
    steps: i32,
}

const I: i32 = 0;

impl FromStr for Command {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(" ") {
            Some((d, s)) => {
                let direction = match d {
                    "forward" => Direction::Forward,
                    "up" => Direction::Up,
                    "down" => Direction::Down,
                    _ => unreachable!(),
                };

                let steps = s.parse::<i32>().unwrap();
                Ok(Command { direction, steps })
            }
            None => {
                unreachable!();
            }
        }
    }
}

fn part1(l: &Vec<Command>) -> i32 {
    let x = l.into_iter().fold((I, I), |(mut x, mut y), v| {
        match v.direction {
            Direction::Forward => x += v.steps,
            Direction::Up => y -= v.steps,
            Direction::Down => y += v.steps,
        }
        (x, y)
    });

    x.0 * x.1
}

fn part2(l: &Vec<Command>) -> i32 {
    let x = l.into_iter().fold((I, I, I), |(mut x, mut y, mut aim), v| {
        match v.direction {
            Direction::Forward => {
                x += v.steps;
                y += v.steps * aim;
            }
            Direction::Up => aim -= v.steps,
            Direction::Down => aim += v.steps,
        }
        (x, y, aim)
    });

    x.0 * x.1
}

fn main() {
    common::time_func(|| {
        let lines = common::read_input::<Command>("input/day02.data");

        println!("Part01: {}", part1(&lines));
        println!("Part02: {}", part2(&lines));
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let lines = common::read_input::<Command>("input/day02.test");

        assert_eq!(part1(&lines), 150);
    }
    #[test]
    fn test_part2() {
        let lines = common::read_input::<Command>("input/day02.test");

        assert_eq!(part2(&lines), 900);
    }
}
