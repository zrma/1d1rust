use crate::utils::io::{read_line, read_value};
use num::ToPrimitive;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1027(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_value(read_line(reader));
    let heights: Vec<i32> = read_line(reader)
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse value"))
        .collect();

    let ans = (0..n).map(|i| count_visible(i, &heights)).max().unwrap();
    write!(writer, "{}", ans).expect("write! should work");
}

fn count_visible(i: usize, heights: &[i32]) -> usize {
    let cnt1 = count_slope((0..i).rev(), i, heights, -1.0);
    let cnt2 = count_slope(i + 1..heights.len(), i, heights, 1.0);
    cnt1 + cnt2
}

fn count_slope(iter: impl Iterator<Item = usize>, i: usize, heights: &[i32], sign: f64) -> usize {
    let mut cnt = 0;
    let mut max_slope = f64::NEG_INFINITY;
    for j in iter {
        let x1 = i.to_f64().expect("Failed to convert i to f64");
        let y1 = heights[i]
            .to_f64()
            .expect("Failed to convert height to f64");
        let x2 = j.to_f64().expect("Failed to convert j to f64");
        let y2 = heights[j]
            .to_f64()
            .expect("Failed to convert height to f64");

        let slope = calc_slope(x1, y1, x2, y2) * sign;
        if slope > max_slope {
            cnt += 1;
            max_slope = slope;
        }
    }
    cnt
}

fn calc_slope(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    (y1 - y2) / (x1 - x2)
}

// https://www.acmicpc.net/problem/1027
// 고층 건물
#[test]
fn test_solve1027() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "15
1 5 3 2 6 3 2 6 4 2 5 7 3 1 5"
                .to_string(),
            want: "7".to_string(),
        },
        TestData {
            s: "1
10"
            .to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "4
5 5 5 5"
                .to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "5
1 2 7 3 2"
                .to_string(),
            want: "4".to_string(),
        },
        TestData {
            s: "10
1000000000 999999999 999999998 999999997 999999996 1 2 3 4 5"
                .to_string(),
            want: "6".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve1027(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
