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

fn find(l: Vec<u32>, i: usize, cmp: fn(f32, f32) -> i32) -> Vec<u32> {
    let lim: f32 = *&l.len() as f32 / 2.0;

    let b = *&l.clone().into_iter().fold(I, |mut x, val: u32| {
        x += (val >> i) & 1;
        x
    });
    let bit = cmp(b as f32, lim);

    let r: Vec<u32> = l
        .into_iter()
        .filter(|x: &u32| {
            if bit == 0 {
                return !(x >> i) & 1 == 1;
            }
            return x >> i & 1 == 1;
        })
        .collect();

    if r.len() > 1 {
        return find(r, i - 1, cmp);
    }

    return r;
}

fn part2(l: &Vec<String>) -> u32 {
    let list = l
        .into_iter()
        .map(|x| {
            let mut val: u32 = 0;
            for c in x.chars() {
                if c == '1' {
                    val += 1
                }
                val <<= 1;
            }
            val >>= 1;
            val
        })
        .collect::<Vec<u32>>();

    let s: usize = l[0].len() as usize;
    let lo2 = find(list.clone(), s - 1, |b, lim| if b >= lim { 1 } else { 0 });
    let lco = find(list, s - 1, |b, lim| if b >= lim { 0 } else { 1 });

    return lco[0] * lo2[0];
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

    #[test]
    fn test_part2() {
        let r = io::BufReader::new(DATA.as_bytes());
        let lines = common::read_input::<String, &[u8]>(r);

        assert_eq!(part2(&lines), 230);
    }
}
