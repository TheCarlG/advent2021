use advent2021::common;
use std::str::FromStr;

const SIZE: usize = 1000;

const GRID_SIZE: usize = SIZE.pow(2);

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = String;

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
    type Err = String;

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

fn _display_grid(g: &[i32]) {
    for (i, n) in g.iter().enumerate() {
        print!("{} ", n);
        if (i + 1) % SIZE == 0 {
            println!();
        }
    }
    println!();
}

fn find(l: &[Line]) -> (usize, usize) {
    let v = vec![0; GRID_SIZE];

    let (g, g2) = l.iter().fold((v.clone(), v), |(mut g, mut g2), line| {
        let x_diff = (line.start.x as isize - line.end.x as isize) as i32;
        let y_diff = (line.start.y as isize - line.end.y as isize) as i32;

        let diag = x_diff.abs() > 0 && y_diff.abs() > 0;

        let diff = x_diff.abs().max(y_diff.abs());

        let dx = match x_diff {
            i if i < 0 => 1,
            i if i > 0 => -1,
            _ => 0,
        };
        let dy = match y_diff {
            i if i < 0 => 1,
            i if i > 0 => -1,
            _ => 0,
        };

        let sy = line.start.y as isize;
        let sx = line.start.x as isize;

        for i in 0..=diff as isize {
            let x = (sx + (dx * i)) as usize;
            let y = (sy + (dy * i)) as usize;

            if !diag {
                g[y * SIZE + x] += 1;
            }
            g2[y * SIZE + x] += 1;
        }

        (g, g2)
    });
    //_display_grid(&g);
    //_display_grid(&g2);

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
