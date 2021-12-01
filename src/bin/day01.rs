use advent2021::common;

fn depth_measurement_count(l: &Vec<u32>) -> usize {
    l.windows(2).filter(|x| x[0] < x[1]).count()
}

fn depth_measurement_slide_count(l: &Vec<u32>) -> usize {
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

        let p1 = depth_measurement_count(&lines);
        let p2 = depth_measurement_slide_count(&lines);

        println!("Part 1: {}", p1);
        println!("Part 2: {}", p2);
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_depth_measurement_count() {
        let test_data: Vec<u32> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(depth_measurement_count(&test_data), 7);
    }

    #[test]
    fn test_depth_measurement_slide_count() {
        let test_data: Vec<u32> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(depth_measurement_slide_count(&test_data), 5);
    }
}
