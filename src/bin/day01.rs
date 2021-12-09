use advent2021::common;

const DAY: &str = "day01";

fn part1(l: &[u32]) -> usize {
    l.windows(2).filter(|x| x[0] < x[1]).count()
}

fn part2(l: &[u32]) -> usize {
    l.windows(3)
        .fold((u32::MAX, 0_usize), |(prev, i), x| {
            let sum = x[0] + x[1] + x[2];
            (sum, i + if prev < sum { 1 } else { 0 })
        })
        .1
}

fn main() {
    common::time_func(|| {
        let lines = common::read_input::<u32>(DAY, false);

        println!("Part01: {}", part1(&lines));
        println!("Part02: {}", part2(&lines));
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let lines = common::read_input::<u32>(DAY, true);
        assert_eq!(part1(&lines), 7);
    }

    #[test]
    fn test_part2() {
        let lines = common::read_input::<u32>(DAY, true);
        assert_eq!(part2(&lines), 5);
    }
}
