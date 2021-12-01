use advent2021::common;
use itermore::IterMore;
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
            .into_iter()
            .windows::<3>()
            .for_each(|[a, b, c]| {
                if a < b || (i == 0 && b < c) {
                    i += 1;
                }

                let val = a + b + c;
                if prev < val {
                    j += 1;
                }
                prev = val;
            });
        println!("Part 1: {}", i);
        println!("Part 2: {}", j);
    });
}
