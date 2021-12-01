use advent2021::common;

fn main() {
    common::time_func(|| {
        let lines = common::read_input("input/day01.txt");

        let p1 = lines.windows(2).filter(|x| x[0] < x[1]).count();
        let p2 = lines
            .windows(3)
            .fold((u32::MAX, 0 as u32), |(prev, i), x| {
                let sum = x[0] + x[1] + x[2];
                (sum, i + if prev < sum { 1 } else { 0 })
            })
            .1;

        println!("Part 1: {}", p1);
        println!("Part 2: {}", p2);
    });
}
