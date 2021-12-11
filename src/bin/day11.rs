use std::{collections::HashSet, fs};

use advent2021::common;

const DAY: &str = "day11";

const SIZE: usize = 10;

type Point = (usize, usize);

struct Grid {
    grid: [[i32; SIZE]; SIZE],
}

impl Grid {
    fn flash(&mut self, start: Point, flashed: &mut HashSet<Point>) -> i32 {
        let (x, y) = start;
        self.grid[y][x] = 0;
        flashed.insert(start);

        let dirs: [(isize, isize); 8] = [
            (-1, -1),
            (1, -1),
            (-1, 1),
            (0, -1),
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 0),
        ];

        let mut res = 0;
        for p in dirs {
            let x1 = x as isize + p.0;
            let y1 = y as isize + p.1;

            if (x1 >= 0 && x1 < SIZE as isize)
                && (y1 >= 0 && y1 < SIZE as isize)
                && !flashed.contains(&(x1 as usize, y1 as usize))
            {
                self.grid[y1 as usize][x1 as usize] += 1;
                if self.grid[y1 as usize][x1 as usize] > 9 {
                    res += self.flash((x1 as usize, y1 as usize), flashed);
                }
            }
        }

        res + 1
    }
}

impl Default for Grid {
    fn default() -> Self {
        Grid {
            grid: [[0; SIZE]; SIZE],
        }
    }
}

fn part1(g: &mut Grid) -> i32 {
    let mut res = 0;

    for _ in 0..100 {
        let mut flashed: HashSet<Point> = HashSet::new();

        for y in 0..SIZE {
            for x in 0..SIZE {
                if !flashed.contains(&(x, y)) {
                    g.grid[y][x] += 1;
                    if g.grid[y][x] > 9 {
                        res += g.flash((x, y), &mut flashed);
                    }
                }
            }
        }
    }

    res
}

fn part2(g: &mut Grid) -> i32 {
    let mut res = 0;

    loop {
        res += 1;
        let mut flashed: HashSet<Point> = HashSet::new();

        for y in 0..SIZE {
            for x in 0..SIZE {
                if !flashed.contains(&(x, y)) {
                    g.grid[y][x] += 1;
                    if g.grid[y][x] > 9 {
                        g.flash((x, y), &mut flashed);
                    }
                }
            }
        }

        if g.grid
            .iter()
            .fold(0, |acc, row| acc + row.iter().sum::<i32>())
            == 0
        {
            break;
        }
    }

    res
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
        let mut grid = read_input(DAY, false);

        println!("Part01: {}", part1(&mut grid));
        println!("Part02: {}", part2(&mut grid) + 100);
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut grid = read_input(DAY, true);
        assert_eq!(part1(&mut grid), 1656);
    }

    #[test]
    fn test_part2() {
        let mut grid = read_input(DAY, true);
        assert_eq!(part2(&mut grid), 195);
    }
}
