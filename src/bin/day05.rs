use advent2021::common;
use std::str::FromStr;
use std::string::ParseError;

const SIZE: usize = 1000;
const GRID_SIZE: usize = SIZE.pow(2);

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(',') {
            Some((x, y)) => Ok(Point {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            }),
            None => {
                unreachable!();
            }
        }
    }
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl FromStr for Line {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.replace(" -> ", "-").split_once('-') {
            Some((start, end)) => Ok(Line {
                start: start.parse::<Point>().unwrap(),
                end: end.parse::<Point>().unwrap(),
            }),
            None => {
                unreachable!();
            }
        }
    }
}

fn _display_grid(g: Vec<i32>) {
    for (i, n) in g.iter().enumerate() {
        print!("{} ", n);
        if (i + 1) % SIZE == 0 {
            println!();
        }
    }
    println!();
}

fn find(l: &[Line]) -> (usize, usize) {
    let mut v = Vec::new();
    v.resize(GRID_SIZE as usize, 0);

    let (g, g2) = l.iter().fold((v.clone(), v), |(mut g, mut g2), line| {
        if line.start.x != line.end.x && line.start.y != line.end.y {
            let mut x: isize = 1;
            let diff = if line.start.x > line.end.x {
                x = -1;
                line.start.x - line.end.x
            } else {
                line.end.x - line.start.x
            };
            let mut y: isize = 1;
            if line.start.y > line.end.y {
                y = -1;
            }
            for i in 0..=diff as isize {
                let pos = SIZE as isize * (line.start.y as isize + (y * i))
                    + (line.start.x as isize + (x * i));
                g2[pos as usize] += 1;
            }
        } else {
            let x_range = if line.start.x > line.end.x {
                line.end.x..=line.start.x
            } else {
                line.start.x..=line.end.x
            };
            let y_range = if line.start.y > line.end.y {
                line.end.y..=line.start.y
            } else {
                line.start.y..=line.end.y
            };

            for x in x_range {
                for y in y_range.clone() {
                    let pos = SIZE * y + x;
                    g[pos as usize] += 1;
                    g2[pos as usize] += 1;
                }
            }
        }
        (g, g2)
    });
    // _display_grid(g);
    // _display_grid(g2);

    let p1 = g.iter().fold(0, |mut p, cell| {
        if *cell > 1 {
            p += 1
        }
        p
    });
    let p2 = g2.iter().fold(0, |mut p, cell| {
        if *cell > 1 {
            p += 1
        }
        p
    });

    (p1, p2)
}

fn main() {
    common::time_func(|| {
        let lines = common::read_input::<String>("input/day05.data");
        let l = lines
            .iter()
            .map(|row| row.parse::<Line>().unwrap())
            .collect::<Vec<Line>>();
        let (p1, p2) = find(&l);
        println!("Part01: {}", p1);
        println!("Part02: {}", p2);
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let lines = common::read_input::<String>("input/day05.test");
        let l = lines
            .iter()
            .map(|row| row.parse::<Line>().unwrap())
            .collect::<Vec<Line>>();
        let (p1, p2) = find(&l);
        assert_eq!(p1, 5);
        assert_eq!(p2, 12);
    }
}
