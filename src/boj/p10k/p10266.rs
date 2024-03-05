use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve10266(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_value(read_line(reader));
    let mut gear_a = read_n_values(reader, n);
    let mut gear_b = read_n_values(reader, n);

    let ans = is_gear_match_possible(&mut gear_a, &mut gear_b);

    write!(writer, "{}", if ans { "possible" } else { "impossible" }).expect("Failed to write");
}

fn is_gear_match_possible(gear_a: &mut [bool], gear_b: &mut [bool]) -> bool {
    let prefix_function = compute_prefix_function(gear_a);

    let mut j = 0;
    for i in 0..GEAR_TEETH_COUNT * 2 {
        while j > 0 && gear_b[i % GEAR_TEETH_COUNT] != gear_a[j] {
            j = prefix_function[j - 1];
        }
        if gear_b[i % GEAR_TEETH_COUNT] == gear_a[j] {
            j += 1;
            if j == GEAR_TEETH_COUNT {
                return true;
            }
        }
    }

    false
}

fn compute_prefix_function(pattern: &[bool]) -> Vec<usize> {
    let mut prefix_function = vec![0; GEAR_TEETH_COUNT];
    let mut j = 0;
    for i in 1..GEAR_TEETH_COUNT {
        while j > 0 && pattern[i] != pattern[j] {
            j = prefix_function[j - 1];
        }
        if pattern[i] == pattern[j] {
            j += 1;
            prefix_function[i] = j;
        }
    }
    prefix_function
}

fn read_n_values(reader: &mut impl BufRead, n: usize) -> Vec<bool> {
    let mut values = vec![false; GEAR_TEETH_COUNT];
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    for v in line
        .split_whitespace()
        .take(n)
        .map(|v| v.parse::<usize>().unwrap())
    {
        values[v] = true;
    }
    values
}

const GEAR_TEETH_COUNT: usize = 360000;

// https://www.acmicpc.net/problem/10266
// 시계 사진들
#[test]
fn test_solve10266() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "6
1 2 3 4 5 6
7 6 5 4 3 1"
                .to_string(),
            want: "impossible".to_string(),
        },
        TestData {
            s: "2
0 270000
180000 270000"
                .to_string(),
            want: "possible".to_string(),
        },
        TestData {
            s: "7
140 130 110 120 125 100 105
235 205 215 220 225 200 240"
                .to_string(),
            want: "impossible".to_string(),
        },
        TestData {
            s: "7
0 45000 90000 135000 225000 270000 315000
0 45000 90000 135000 180000 225000 315000"
                .to_string(),
            want: "possible".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve10266(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
