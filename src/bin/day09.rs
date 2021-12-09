use std::fs;

use advent2021::common;

const DAY: &str = "day09";

const SIZE: usize = 100;

struct Grid {
    grid: [[i32; SIZE]; SIZE],
}

impl Grid {
    fn lowpoints(&self, width: usize, height: usize) -> Vec<i32> {
        let mut res = Vec::new();
        for y in 0..height {
            for x in 0..width {
                let val = self.grid[y][x];

                if (y > 0) && self.grid[y - 1][x] <= val {
                    continue;
                }
                if (y + 1 < height) && self.grid[y + 1][x] <= val {
                    continue;
                }
                if (x > 0) && self.grid[y][x - 1] <= val {
                    continue;
                }
                if (x + 1 < width) && self.grid[y][x + 1] <= val {
                    continue;
                }
                res.push(val);
            }
        }

        res
    }
}

impl Default for Grid {
    fn default() -> Self {
        Grid {
            grid: [[0; SIZE]; SIZE],
        }
    }
}

fn part1(g: &Grid) -> i32 {
    let low = g.lowpoints(SIZE, SIZE);

    low.iter().map(|v| v + 1).collect::<Vec<i32>>().iter().sum()
}

fn part2(_g: &Grid) -> i32 {
    0
}

fn read_input(day: &str, test: bool) -> Grid {
    let set = if test { "test" } else { "data" };
    let filename = format!("input/{}.{}", day, set);

    let mut g = Grid::default();
    let mut y = 0;

    fs::read_to_string(&filename)
        .unwrap_or_else(|_| panic!("{}", format!("unable to read {:}", filename).as_str()))
        .lines()
        .for_each(|s| {
            for (x, c) in s.chars().enumerate() {
                g.grid[y][x] = c as i32 - '0' as i32;
            }
            y += 1;
        });

    g
}

fn main() {
    common::time_func(|| {
        let grid = read_input(DAY, false);

        let low = grid.lowpoints(SIZE, SIZE);
        let part1: i32 = low.iter().map(|v| v + 1).collect::<Vec<i32>>().iter().sum();
        println!("Part01: {}", part1);
        println!("Part02: {}", part2(&grid));
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let grid = read_input(DAY, true);

        let low = grid.lowpoints(10, 5);

        let sum: i32 = low.iter().map(|v| v + 1).collect::<Vec<i32>>().iter().sum();
        assert_eq!(sum, 15);
    }

    #[test]
    #[ignore]
    fn test_part2() {
        let grid = read_input(DAY, true);
        assert_eq!(part2(&grid), 1134);
    }
}
