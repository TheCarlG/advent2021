use advent2021::common;

fn main() {
    common::time_func(|| {
        let mut prev = u32::MAX;
        let mut i = 0;
        let mut j = 0;

        common::read_input("input/day01.txt")
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
