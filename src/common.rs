use std::time::Instant;
pub fn time_func(f: fn()) {
    let start = Instant::now();
    f();
    let duration = start.elapsed();

    println!("Execution time is: {:?}", duration);
}
