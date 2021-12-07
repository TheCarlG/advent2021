use advent2021::common;

fn calc(mut input: Vec<i32>) -> i32 {
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

fn main() {
    common::time_func(|| {
        let input: Vec<i32> = common::read_input::<String>("input/day07.data")
            .first()
            .unwrap()
            .split(',')
            .map(|v| v.parse::<i32>().unwrap())
            .collect();

        println!("Part01: {}", calc(input.clone()));
        //println!("Part02: {}", calc(&input));
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

        assert_eq!(calc(input), 37);
    }

    #[test]
    #[ignore]
    fn test_part2() {
        let input: Vec<i32> = common::read_input::<String>("input/day07.data")
            .first()
            .unwrap()
            .split(',')
            .map(|v| v.parse::<i32>().unwrap())
            .collect();

        assert_eq!(calc(input), 37);
    }
}
