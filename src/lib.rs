pub mod common {
    use std::fs;
    use std::process;
    use std::str::FromStr;
    use std::time::Instant;

    #[allow(dead_code)]
    pub fn time_func(f: fn()) {
        let start = Instant::now();
        f();
        let duration = start.elapsed();

        println!("Time: {:.7}ms", duration.as_secs_f64() * 1000.0);
    }

    #[allow(dead_code)]
    pub fn read_input<T>(filename: &str) -> Vec<T>
    where
        T: FromStr,
    {
        fs::read_to_string(filename)
            .unwrap_or_else(|_| panic!("{}", format!("unable to read {:}", filename).as_str()))
            .lines()
            .map(|v| -> T {
                match v.parse::<T>() {
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

        #[test]
        fn test_read_input() {
            let lines = read_input::<u32>("input/day01.test");
            assert_eq!(lines[0], 199);

            let lines = read_input::<String>("input/day01.test");
            assert_eq!(lines[0], "199");
        }
    }
}
