use std::fs;
use std::time::Instant;

pub fn time_func(f: fn()) {
    let start = Instant::now();
    f();
    let duration = start.elapsed();

    println!("Execution time is: {:?}", duration);
}

pub fn read_input(input: &str) -> Vec<u32> {
    fs::read_to_string(input)
        .expect("Unable to read input file")
        .lines()
        .map(|v| v.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}
