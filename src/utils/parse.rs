use std::io::{BufRead, Lines};

pub fn str_to_arr(mut lines: Lines<&mut impl BufRead>) -> Vec<i32> {
    let res: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    if res.is_empty() {
        lines
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect()
    } else {
        res
    }
}
