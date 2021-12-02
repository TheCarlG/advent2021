pub mod common {
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::process;
    use std::str::FromStr;
    use std::time::Instant;

    pub fn time_func(f: fn()) {
        let start = Instant::now();
        f();
        let duration = start.elapsed();

        println!("Execution time is: {:?}", duration);
    }

    pub fn read_input<T: FromStr, R: Read>(input: BufReader<R>) -> Vec<T> {
        input
            .lines()
            .map(|v| -> T {
                match v.unwrap().parse::<T>() {
                    Ok(x) => x,
                    Err(_) => {
                        println!("Unable to parse value");
                        process::exit(1);
                    }
                }
            })
            .collect::<Vec<T>>()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const DATA: &str = "199
200
208
210
200
207
240
269
260
263";

        #[test]
        fn test_read_input() {
            let r = BufReader::new(DATA.as_bytes());
            let lines = read_input::<u32, &[u8]>(r);
            assert_eq!(lines[0], 199);

            let r = BufReader::new(DATA.as_bytes());
            let lines = read_input::<String, &[u8]>(r);
            assert_eq!(lines[0], "199");
        }
    }
}
