use std::i32::MAX;

use advent2021::common;

fn part1(mut input: Vec<i32>) -> i32 {
    let n = input.len();

    input.sort_unstable();

    let x = if n % 2 == 1 {
        input[n / 2]
    } else {
        (input[n / 2] + input[(n - 2) / 2]) / 2
    };

    let mut s = 0;
    (0..n).for_each(|i| {
        s += (input[i] - x).abs();
    });

    s
}

fn part2(mut input: Vec<i32>) -> i32 {
    input.sort_unstable();

    let n = input.len();

    let low = input[0];
    let mut s = MAX;
    let mut curr = 0;

    (low..=input[n - 1]).for_each(|i| {
        (0..n).for_each(|j| {
            let v = (input[j] - i).abs();
            curr += v * (1 + v) / 2;
        });

        if curr < s {
            s = curr;
        }

        curr = 0;
    });

    s
}

fn main() {
    common::time_func(|| {
        let input: Vec<i32> = common::read_input::<String>("input/day07.data")
            .first()
            .unwrap()
            .split(',')
            .map(|v| v.parse::<i32>().unwrap())
            .collect();

        println!("Part01: {}", part1(input.clone()));
        println!("Part02: {}", part2(input));
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input: Vec<i32> = common::read_input::<String>("input/day07.test")
            .first()
            .unwrap()
            .split(',')
            .map(|v| v.parse::<i32>().unwrap())
            .collect();

        assert_eq!(part1(input), 37);
    }

    #[test]
    fn test_part2() {
        let input: Vec<i32> = common::read_input::<String>("input/day07.test")
            .first()
            .unwrap()
            .split(',')
            .map(|v| v.parse::<i32>().unwrap())
            .collect();

        assert_eq!(part2(input), 168);
    }
}
