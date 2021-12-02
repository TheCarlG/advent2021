#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Forward,
    Backward,
}

use advent2021::common;

fn process_input(l: Vec<String>) -> Vec<(Direction, i32)> {
    l.into_iter()
        .map(|x| {
            let parts: Vec<&str> = x.split(" ").collect();

            let direction = match parts[0] {
                "forward" => Direction::Forward,
                "backward" => Direction::Backward,
                "up" => Direction::Up,
                "down" => Direction::Down,
                _ => unreachable!(),
            };

            let steps = parts[1].parse::<i32>().unwrap();

            (direction, steps)
        })
        .collect::<Vec<(Direction, i32)>>()
}

fn part1(l: &Vec<(Direction, i32)>) -> i32 {
    let x = l
        .into_iter()
        .fold((0 as i32, 0 as i32), |(mut x, mut y), v| {
            match v.0 {
                Direction::Forward => x += v.1,
                Direction::Backward => x -= v.1,
                Direction::Up => y -= v.1,
                Direction::Down => y += v.1,
            }
            (x, y)
        });

    x.0 * x.1
}

fn part2(l: &Vec<(Direction, i32)>) -> i32 {
    let x = l.into_iter().fold(
        (0 as i32, 0 as i32, 0 as i32),
        |(mut x, mut y, mut aim), v| {
            match v.0 {
                Direction::Forward => {
                    x += v.1;
                    y += v.1 * aim;
                }
                Direction::Backward => {
                    x -= v.1;
                    y -= v.1 * aim;
                }
                Direction::Up => aim -= v.1,
                Direction::Down => aim += v.1,
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
        let test_data = process_input(vec![
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
        let test_data = process_input(vec![
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
