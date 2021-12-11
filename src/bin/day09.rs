use std::{collections::HashSet, fs};

use advent2021::common;

const DAY: &str = "day09";

const SIZE: usize = 100;

type Point = (usize, usize);

struct Grid {
    grid: [[i32; SIZE]; SIZE],
    view_limit: Point,
}

impl Grid {
    fn lowpoints(&self) -> Vec<(i32, Point)> {
        let mut res = Vec::new();
        let (width, height) = self.view_limit;

        let dirs: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        for y in 0..height {
            'outer: for x in 0..width {
                let val = self.grid[y][x];
                for p in dirs {
                    let x1 = x as isize + p.0;
                    let y1 = y as isize + p.1;

                    if (x1 >= 0 && x1 < width as isize)
                        && (y1 >= 0 && y1 < height as isize)
                        && self.grid[y1 as usize][x1 as usize] <= val
                    {
                        continue 'outer;
                    }
                }

                res.push((val, (x, y)));
            }
        }

        res
    }

    fn find_path(&self, start: Point, visited: &mut HashSet<Point>) -> i32 {
        let (width, height) = self.view_limit;
        visited.insert(start);

        let mut res = 0;
        let (x, y) = start;
        let dirs: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        for p in dirs {
            let x1 = x as isize + p.0;
            let y1 = y as isize + p.1;

            if (x1 >= 0 && x1 < width as isize)
                && (y1 >= 0 && y1 < height as isize)
                && !visited.contains(&(x1 as usize, y1 as usize))
                && self.grid[y1 as usize][x1 as usize] < 9
            {
                res += self.find_path((x1 as usize, y1 as usize), visited);
            }
        }

        res + 1
    }

    fn basins(&self, lowpoints: &[(i32, Point)]) -> Vec<i32> {
        let mut res = Vec::new();

        let mut visited: HashSet<Point> = HashSet::new();
        lowpoints.iter().for_each(|p| {
            let val = self.find_path(p.1, &mut visited);
            if val > 0 {
                res.push(val);
            }
        });

        res.sort_unstable();

        res
    }
}

impl Default for Grid {
    fn default() -> Self {
        Grid {
            grid: [[0; SIZE]; SIZE],
            view_limit: (SIZE, SIZE),
        }
    }
}

fn part1(v: &[(i32, Point)]) -> i32 {
    v.iter().map(|v| v.0 + 1).collect::<Vec<i32>>().iter().sum()
}

fn part2(v: Vec<i32>) -> i32 {
    v.iter().rev().take(3).product()
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

        let low = grid.lowpoints();
        println!("Part 1: {}", part1(&low));

        let basins = grid.basins(&low);
        println!("Part 2: {}", part2(basins));
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut grid = read_input(DAY, true);
        grid.view_limit = (10, 5);

        let low = grid.lowpoints();

        let sum: i32 = low
            .iter()
            .map(|v| v.0 + 1)
            .collect::<Vec<i32>>()
            .iter()
            .sum();
        assert_eq!(sum, 15);
    }

    #[test]
    fn test_part2() {
        let mut grid = read_input(DAY, true);
        grid.view_limit = (10, 5);

        let low = grid.lowpoints();
        let basins = grid.basins(&low);
        assert_eq!(part2(basins), 1134);
    }
}
