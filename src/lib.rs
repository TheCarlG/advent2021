pub mod common {
    use std::fs;
    use std::process;
    use std::str::FromStr;
    use std::time::Instant;

    pub fn time_func(f: fn()) {
        let start = Instant::now();
        f();
        let duration = start.elapsed();

        println!("Execution time is: {:?}", duration);
    }

    pub fn read_input<T: FromStr>(input: &str) -> Vec<T> {
        fs::read_to_string(input)
            .expect("Unable to read input file")
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
            let v1 = read_input::<u32>("testdata/input_u32.txt");
            assert_eq!(v1.len(), 10);
            assert_eq!(v1[0], 199);

            let v2 = read_input::<String>("testdata/input_u32.txt");
            assert_eq!(v2.len(), 10);
            assert_eq!(v2[0], "199");
        }
    }
}
