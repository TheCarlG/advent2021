use advent2021::common;
use std::fs::File;
use std::io;

const I: u32 = 0;

fn part1(l: &Vec<String>) -> u32 {
    let lim: u32 = (l.len() / 2).try_into().unwrap();
    let s: usize = l[0].len() as usize;

    let (mut gamma, mut mask) = l
        .into_iter()
        .fold(vec![I; s], |mut arr, val: &String| {
            for (i, c) in val.chars().enumerate() {
                if c == '1' {
                    arr[i] += 1;
                }
            }
            arr
        })
        .into_iter()
        .fold((I, I), |(mut gamma, mut mask), x| {
            if x > lim {
                gamma += 1;
            }
            mask += 1;
            mask <<= 1;
            gamma <<= 1;
            (gamma, mask)
        });

    mask >>= 1;
    gamma >>= 1;

    gamma * (gamma ^ mask) as u32
}

fn part2(_l: &Vec<String>) -> u32 {
    return 0;
}

fn main() {
    common::time_func(|| match File::open("input/day03.txt") {
        Ok(f) => {
            let r = io::BufReader::new(f);
            let lines = common::read_input::<String, File>(r);
            println!("Part01: {}", part1(&lines));
            println!("Part02: {}", part2(&lines));
        }
        Err(_) => unreachable!(),
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn test_part1() {
        let r = io::BufReader::new(DATA.as_bytes());
        let lines = common::read_input::<String, &[u8]>(r);

        assert_eq!(part1(&lines), 198);
    }

    // #[test]
    // fn test_part2() {
    //     let r = io::BufReader::new(DATA.as_bytes());
    //     let lines = common::read_input::<u32, &[u8]>(r);

    //     assert_eq!(part2(&lines), 5);
    // }
}
