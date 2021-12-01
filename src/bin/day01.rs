use advent2021::common;
use std::fs;

fn main() {
    common::time_func(|| {
        let mut prev = u32::MAX;
        let mut i = 0;
        let mut j = 0;

        fs::read_to_string(r#"input/day01.txt"#)
            .expect("Unable to read day01.txt")
            .lines()
            .map(|v| v.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
            .windows(3)
            .for_each(|x| {
                if x[0] < x[1] || (i == 0 && x[1] < x[2]) {
                    i += 1;
                }

                let val = x.into_iter().sum();
                if prev < val {
                    j += 1;
                }
                prev = val;
            });

        println!("Part 1: {}", i);
        println!("Part 2: {}", j);
    });
}
