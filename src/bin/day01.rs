use advent2021::common;

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
    common::time_func(|| {
        let lines = common::read_input::<u32>("input/day01.txt");

        println!("Part 1: {}", part1(&lines));
        println!("Part 2: {}", part2(&lines));
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test_data: Vec<u32> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(part1(&test_data), 7);
    }

    #[test]
    fn test_part2() {
        let test_data: Vec<u32> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(part2(&test_data), 5);
    }
}
