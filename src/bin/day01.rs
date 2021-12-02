use advent2021::common;
use std::fs::File;
use std::io;

fn part1(l: &Vec<u32>) -> usize {
    l.windows(2).filter(|x| x[0] < x[1]).count()
}

fn part2(l: &Vec<u32>) -> usize {
    l.windows(3)
        .fold((u32::MAX, 0 as usize), |(prev, i), x| {
            let sum = x[0] + x[1] + x[2];
            (sum, i + if prev < sum { 1 } else { 0 })
        })
        .1
}

fn main() {
    common::time_func(|| match File::open("input/day01.txt") {
        Ok(f) => {
            let r = io::BufReader::new(f);
            let lines = common::read_input::<u32, File>(r);

            println!("Part 1: {}", part1(&lines));
            println!("Part 2: {}", part2(&lines));
        }
        Err(_) => unreachable!(),
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = "199
200
208
210
200
207
240
269
260
263";

        let r = io::BufReader::new(data.as_bytes());
        let lines = common::read_input::<u32, &[u8]>(r);

        assert_eq!(part1(&lines), 7);
    }

    #[test]
    fn test_part2() {
        let data = "199
200
208
210
200
207
240
269
260
263";

        let r = io::BufReader::new(data.as_bytes());
        let lines = common::read_input::<u32, &[u8]>(r);

        assert_eq!(part2(&lines), 5);
    }
}
