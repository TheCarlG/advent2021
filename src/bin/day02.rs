#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Forward,
    Backward,
}

use advent2021::common;

#[derive(Debug)]
struct Command {
    direction: Direction,
    steps: i32,
}

impl Command {
    fn new(direction_str: &str, steps: i32) -> Self {
        let direction = match direction_str {
            "forward" => Direction::Forward,
            "backward" => Direction::Backward,
            "up" => Direction::Up,
            "down" => Direction::Down,
            _ => {
                println!("Invalid direction: {}", direction_str);
                std::process::exit(1);
            }
        };
        Self { direction, steps }
    }
}

fn process_input(l: Vec<String>) -> Vec<Command> {
    l.into_iter()
        .map(|x| {
            let parts: Vec<&str> = x.split(" ").collect();
            Command::new(parts[0], parts[1].parse::<i32>().unwrap())
        })
        .collect::<Vec<Command>>()
}

fn part1(l: &Vec<Command>) -> i32 {
    let x = l
        .into_iter()
        .fold((0 as i32, 0 as i32), |(mut x, mut y), v| {
            match v.direction {
                Direction::Forward => x += v.steps,
                Direction::Backward => x -= v.steps,
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
                Direction::Backward => {
                    x -= v.steps;
                    y -= v.steps * aim;
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
    common::time_func(|| {
        let lines = process_input(common::read_input::<String>("input/day02.txt"));

        println!("Part 1: {}", part1(&lines));
        println!("Part 2: {}", part2(&lines));
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test_data: Vec<Command> = process_input(vec![
            "forward 5".to_string(),
            "down 5".to_string(),
            "forward 8".to_string(),
            "up 3".to_string(),
            "down 8".to_string(),
            "forward 2".to_string(),
        ]);

        assert_eq!(part1(&test_data), 150);
    }

    #[test]
    fn test_part2() {
        let test_data: Vec<Command> = process_input(vec![
            "forward 5".to_string(),
            "down 5".to_string(),
            "forward 8".to_string(),
            "up 3".to_string(),
            "down 8".to_string(),
            "forward 2".to_string(),
        ]);

        assert_eq!(part2(&test_data), 900);
    }
}
