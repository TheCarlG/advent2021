use std::fmt::Display;
use std::str::FromStr;

use advent2021::common;

#[derive(Clone, Debug, Copy)]
struct Fish {
    timer: i32,
    count: u64,
}

impl Fish {
    fn default(cnt: u64) -> Self {
        Self {
            timer: 8,
            count: cnt,
        }
    }
    fn new(cnt: u64) -> Self {
        Self {
            timer: 6,
            count: cnt,
        }
    }

    fn decrease(&mut self) {
        self.timer -= 1;
    }
}

impl Display for Fish {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.timer, self.count)
    }
}

impl FromStr for Fish {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Fish {
            timer: s.parse::<i32>().unwrap(),
            count: 1,
        })
    }
}

fn simulate(mut data: Vec<Fish>, days: i32) -> Vec<Fish> {
    for _ in 1..=days {
        let mut cnt: u64 = 0;
        data = data
            .iter()
            .filter(|f| {
                if f.timer == -1 {
                    cnt += f.count;
                    false
                } else {
                    true
                }
            })
            .cloned()
            .collect();

        if cnt > 0 {
            data.push(Fish::new(cnt));
        }
        for i in 0..data.len() {
            let mut f = data[i];
            if f.timer == 0 {
                data.push(Fish::default(f.count));
            }
            f.decrease();
            data[i] = f;
        }
    }

    data
}

fn main() {
    common::time_func(|| {
        let data: Vec<Fish> = common::read_input::<String>("input/day06.data")
            .first()
            .unwrap()
            .split(',')
            .map(|v| v.parse::<Fish>().unwrap())
            .collect();

        let data = simulate(data, 80);
        let mut cnt = data.iter().fold(0_u64, |cnt, f| cnt + (f.count as u64));
        println!("Part01: {}", cnt);

        let data = simulate(data, 256 - 80);
        cnt = data.iter().fold(0_u64, |cnt, f| cnt + (f.count as u64));
        println!("Part02: {}", cnt);
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part11() {
        let data: Vec<Fish> = common::read_input::<String>("input/day06.test")
            .first()
            .unwrap()
            .split(',')
            .map(|v| v.parse::<Fish>().unwrap())
            .collect();

        let data = simulate(data, 18);
        let cnt = data.iter().fold(0_u64, |cnt, f| cnt + (f.count as u64));
        assert_eq!(cnt, 26);
    }

    #[test]
    fn test_part1() {
        let data: Vec<Fish> = common::read_input::<String>("input/day06.test")
            .first()
            .unwrap()
            .split(',')
            .map(|v| v.parse::<Fish>().unwrap())
            .collect();

        let data = simulate(data, 80);
        let cnt = data.iter().fold(0_u64, |cnt, f| cnt + (f.count as u64));
        assert_eq!(cnt, 5934);
    }

    #[test]
    fn test_part2() {
        let data: Vec<Fish> = common::read_input::<String>("input/day06.test")
            .first()
            .unwrap()
            .split(',')
            .map(|v| v.parse::<Fish>().unwrap())
            .collect();

        //let data = simulate(data, 80);
        let data = simulate(data, 256);
        let cnt = data.iter().fold(0, |cnt, f| cnt + f.count as u64);
        assert_eq!(cnt, 26984457539);
    }
}
