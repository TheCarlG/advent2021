use advent2021::common;

const DAY: &str = "day03";

fn part1(l: &[String]) -> u32 {
    let mid: u32 = (l.len() / 2).try_into().unwrap();
    let s: usize = l[0].len() as usize;

    let gamma = l
        .iter()
        .fold(vec![0_u32; s], |mut acc, val: &String| {
            for (i, c) in val.chars().enumerate() {
                if c == '1' {
                    acc[i] += 1;
                }
            }
            acc
        })
        .iter()
        .fold(0_u32, |mut gamma, v| {
            if v > &mid {
                gamma += 1;
            }
            gamma << 1
        })
        >> 1;

    let mask = (1 << s) - 1;

    gamma * (gamma ^ mask)
}

fn find(l: Vec<u32>, i: usize, least_common: bool) -> Vec<u32> {
    let mid: f32 = l.len() as f32 / 2.0;

    let ones = l
        .clone()
        .into_iter()
        .fold(0_u32, |x, val: u32| x + ((val >> i) & 1)) as f32;

    let r: Vec<u32> = l
        .into_iter()
        .filter(|x: &u32| {
            if (least_common && ones < mid) || (!least_common && ones >= mid) {
                x >> i & 1 == 1
            } else {
                !(x >> i) & 1 == 1
            }
        })
        .collect();

    if r.len() > 1 {
        find(r, i - 1, least_common)
    } else {
        r
    }
}

fn part2(l: &[String]) -> u32 {
    let list = l
        .iter()
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

    let s: usize = l[0].len() as usize - 1;
    let o2 = find(list.clone(), s, false)[0];
    let co = find(list, s, true)[0];

    co * o2
}

fn main() {
    common::time_func(|| {
        let lines = common::read_input::<String>(DAY, false);

        println!("Part 1: {}", part1(&lines));
        println!("Part 2: {}", part2(&lines));
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let lines = common::read_input::<String>(DAY, true);
        assert_eq!(part1(&lines), 198);
    }

    #[test]
    fn test_part2() {
        let lines = common::read_input::<String>(DAY, true);

        assert_eq!(part2(&lines), 230);
    }
}
