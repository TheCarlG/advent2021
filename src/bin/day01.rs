use advent2021::common;

fn main() {
    common::time_func(|| {
        let l = common::read_input("input/day01.txt");

        let mut prev = u32::MAX;
        let mut j = 0;
        let mut i = if l[l.len() - 1] > l[l.len() - 2] {
            1
        } else {
            0
        };
        l.windows(3).for_each(|x| {
            if x[0] < x[1] {
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
