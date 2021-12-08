use std::collections::HashMap;

use advent2021::common;

fn part1(input: Vec<(Vec<&str>, Vec<&str>)>) -> i32 {
    let mut m: HashMap<usize, i32> = HashMap::new();

    input.iter().for_each(|v| {
        v.1.iter().for_each(|k| {
            let k = (**k).len();
            m.entry(k).and_modify(|val| *val += 1).or_insert(1);
        })
    });
    m[&2] + m[&3] + m[&4] + m[&7]
}

fn remove(s1: &String, s2: &String) -> String {
    s1.chars()
        .filter(|x| !s2.contains(&x.to_string()))
        .collect()
}

fn merge(s1: &String, s2: &String) -> String {
    let v1: Vec<char> = s1.chars().collect();
    let v2: Vec<char> = s2.chars().collect();

    let mut v = v1.clone();

    v.extend(v2);
    v.sort_unstable();
    v.dedup();

    v.into_iter().collect()
}

fn part2(input: Vec<(Vec<&str>, Vec<&str>)>) -> i32 {
    let mut result = 0;
    input.iter().for_each(|v| {
        let mut m1: HashMap<&str, String> = HashMap::new();
        let mut m2: HashMap<String, &str> = HashMap::new();

        let v0: Vec<String> =
            v.0.iter()
                .map(|s| {
                    let mut v: Vec<char> = s.chars().collect();
                    v.sort_unstable();
                    v.into_iter().collect()
                })
                .collect();

        let v1: Vec<String> =
            v.1.iter()
                .map(|s| {
                    let mut v: Vec<char> = s.chars().collect();
                    v.sort_unstable();
                    v.into_iter().collect()
                })
                .collect();

        v0.iter().for_each(|k| {
            let l = (k).len();
            if l == 2 {
                m1.insert("1", String::from(k));
                m2.insert(String::from(k), "1");
            }
            if l == 4 {
                m1.insert("4", String::from(k));
                m2.insert(String::from(k), "4");
            }
            if l == 3 {
                m1.insert("7", String::from(k));
                m2.insert(String::from(k), "7");
            }
            if l == 7 {
                m1.insert("8", String::from(k));
                m2.insert(String::from(k), "8");
            }
        });
        let mut i = 0;
        while m1.len() < 10 || m2.len() < 10 {
            i += 1;
            if i >= 1000 {
                break;
            }
            for k in v0.iter() {
                let l = k.len();
                if l == 5 {
                    if !m1.contains_key(&"2") {
                        let s = String::from(k);
                        let v = remove(&s, &m1[&"4"]);
                        let v2 = remove(&s, &merge(&m1[&"7"], &m1[&"4"]));

                        if v.len() == 3 && v2.len() == 2 {
                            m2.insert(s.clone(), "2");
                            m1.insert("2", s);
                        }
                    }
                    if !m1.contains_key(&"3") {
                        let s = String::from(k);
                        let v = remove(&s, &m1[&"7"]);
                        let v2 = remove(&s, &m1[&"1"]);
                        if v.len() == 2 && v2.len() == 3 {
                            m2.insert(s.clone(), "3");
                            m1.insert("3", s);
                        }
                    }
                    if !m1.contains_key(&"5") {
                        let s = String::from(k);
                        let v = remove(&s, &m1[&"4"]);
                        let tmp = remove(&m1[&"8"], &m1[&"7"]);
                        let v2 = remove(&s, &tmp);
                        if v.len() == 2 && v2.len() == 2 {
                            m2.insert(s.clone(), "5");
                            m1.insert("5", s);
                        }
                    }
                } else if l == 6 {
                    if !m1.contains_key(&"0") {
                        let s = String::from(k);
                        let v = remove(&s, &m1[&"7"]);
                        let v2 = remove(&s, &m1[&"4"]);
                        if v.len() == 3 && v2.len() == 3 {
                            m2.insert(s.clone(), "0");
                            m1.insert("0", s);
                        }
                    }
                    if !m1.contains_key(&"6") {
                        let s = String::from(k);
                        let v = remove(&s, &m1[&"7"]);
                        let v2 = remove(&s, &m1[&"4"]);
                        if v.len() == 4 && v2.len() == 3 {
                            m2.insert(s.clone(), "6");
                            m1.insert("6", s);
                        }
                    }
                    if !m1.contains_key(&"9") {
                        let s = String::from(k);
                        let v = remove(&s, &m1[&"4"]);
                        if v.len() == 2 {
                            m2.insert(s.clone(), "9");
                            m1.insert("9", s);
                        }
                    }
                }
            }
        }
        let mut num = "".to_string();
        for w in v1.iter() {
            num.push_str(m2[w]);
        }
        result += num.parse::<i32>().unwrap();
    });

    result
}

fn main() {
    common::time_func(|| {
        let input = common::read_input::<String>("input/day08.data");
        let input: Vec<(Vec<&str>, Vec<&str>)> = input
            .iter()
            .map(|r| {
                let t = r.split_once('|').unwrap();
                let p1: Vec<&str> = t.0.trim().split(' ').collect();
                let p2: Vec<&str> = t.1.trim().split(' ').collect();

                (p1, p2)
            })
            .collect();

        println!("Part01: {}", part1(input.clone()));
        println!("Part02: {}", part2(input));
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove() {
        let s1 = "abcdefg".to_string();
        let s2 = "acdf".to_string();
        assert_eq!(remove(&s1, &s2), "beg");

        let s1 = "gf".to_string();
        let s2 = "g".to_string();
        assert_eq!(remove(&s1, &s2), "f");
    }

    #[test]
    fn test_merge() {
        let s2 = "acdfijk".to_string();
        let s1 = "abcdefg".to_string();
        assert_eq!(merge(&s1, &s2), "abcdefgijk");
    }

    #[test]
    fn test_part1() {
        let input = common::read_input::<String>("input/day08.test");
        let input: Vec<(Vec<&str>, Vec<&str>)> = input
            .iter()
            .map(|r| {
                let t = r.split_once('|').unwrap();
                let p1: Vec<&str> = t.0.trim().split(' ').collect();
                let p2: Vec<&str> = t.1.trim().split(' ').collect();

                (p1, p2)
            })
            .collect();

        assert_eq!(part1(input), 26);
    }

    #[test]
    fn test_part2() {
        let input = common::read_input::<String>("input/day08.test");
        let input: Vec<(Vec<&str>, Vec<&str>)> = input
            .iter()
            .map(|r| {
                let t = r.split_once('|').unwrap();
                let p1: Vec<&str> = t.0.trim().split(' ').collect();
                let p2: Vec<&str> = t.1.trim().split(' ').collect();

                (p1, p2)
            })
            .collect();

        assert_eq!(part2(input), 61229);
    }
}
