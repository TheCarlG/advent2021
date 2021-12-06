use advent2021::common;

fn simulate(input: &[u128], days: i32) -> [u128; 9] {
    let mut data = [0_u128; 9];
    for i in input {
        data[*i as usize] += 1;
    }

    for _ in 1..=days {
        let new = data[0];
        for i in 0..=7 {
            data[i] = data[i + 1];
        }
        data[6] += new;
        data[8] = new;
    }

    data
}

fn main() {
    common::time_func(|| {
        let input: Vec<u128> = common::read_input::<String>("input/day06.data")
            .first()
            .unwrap()
            .split(',')
            .map(|v| v.parse::<u128>().unwrap())
            .collect();

        let data = simulate(&input, 80);
        let cnt: u128 = data.iter().sum();
        println!("Part01: {}", cnt);

        let data = simulate(&input, 256);
        let cnt: u128 = data.iter().sum();
        println!("Part02: {}", cnt);
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part11() {
        let input: Vec<u128> = common::read_input::<String>("input/day06.test")
            .first()
            .unwrap()
            .split(',')
            .map(|v| v.parse::<u128>().unwrap())
            .collect();

        let data = simulate(&input, 18);
        let cnt: u128 = data.iter().sum();
        assert_eq!(cnt, 26);
    }

    #[test]
    fn test_part1() {
        let input: Vec<u128> = common::read_input::<String>("input/day06.test")
            .first()
            .unwrap()
            .split(',')
            .map(|v| v.parse::<u128>().unwrap())
            .collect();

        let data = simulate(&input, 80);
        let cnt: u128 = data.iter().sum();
        assert_eq!(cnt, 5934);
    }

    #[test]
    fn test_part2() {
        let input: Vec<u128> = common::read_input::<String>("input/day06.test")
            .first()
            .unwrap()
            .split(',')
            .map(|v| v.parse::<u128>().unwrap())
            .collect();

        let data = simulate(&input, 256);
        let cnt: u128 = data.iter().sum();
        assert_eq!(cnt, 26984457539);
    }
}
