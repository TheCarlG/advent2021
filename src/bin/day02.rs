use advent2021::common;
use std::fs::File;
use std::io;
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

impl FromStr for Command {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (d, s) = s.split_once(" ").unwrap();

        let direction = match d {
            "forward" => Direction::Forward,
            "up" => Direction::Up,
            "down" => Direction::Down,
            _ => unreachable!(),
        };

        let steps = s.parse::<i32>().unwrap();

        Ok(Command { direction, steps })
    }
}

fn part1(l: &Vec<Command>) -> i32 {
    let x = l
        .into_iter()
        .fold((0 as i32, 0 as i32), |(mut x, mut y), v| {
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
    let x = l.into_iter().fold(
        (0 as i32, 0 as i32, 0 as i32),
        |(mut x, mut y, mut aim), v| {
            match v.direction {
                Direction::Forward => {
                    x += v.steps;
                    y += v.steps * aim;
                }
                Direction::Up => aim -= v.steps,
                Direction::Down => aim += v.steps,
            }
            (x, y, aim)
        },
    );

    x.0 * x.1
}

fn main() {
    common::time_func(|| match File::open("input/day02.txt") {
        Ok(f) => {
            let r = io::BufReader::new(f);
            let lines = common::read_input::<Command, File>(r);

            println!("Part01: {}", part1(&lines));
            println!("Part02: {}", part2(&lines));
        }
        Err(_) => unreachable!(),
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn test_part1() {
        let r = io::BufReader::new(DATA.as_bytes());
        let lines = common::read_input::<Command, &[u8]>(r);

        assert_eq!(part1(&lines), 150);
    }
    #[test]
    fn test_part2() {
        let r = io::BufReader::new(DATA.as_bytes());
        let lines = common::read_input::<Command, &[u8]>(r);

        assert_eq!(part2(&lines), 900);
    }
}
