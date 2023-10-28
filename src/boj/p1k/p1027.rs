use crate::utils::io::read_line;
use std::io::{BufRead, Write};

macro_rules! count_slope {
    ($iter:expr, $i:expr, $heights:expr, $sign:expr, $cnt:expr) => {{
        let mut max_slope = -1e100;
        for j in $iter {
            let slope = calc_slope($i, $heights[$i], j, $heights[j]) * $sign;
            if slope > max_slope {
                $cnt += 1;
                max_slope = slope;
            }
        }
    }};
}

#[allow(dead_code)]
fn solve1027(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_line(reader).parse::<usize>().unwrap();
    let heights: Vec<i32> = read_line(reader)
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let ans = (0..n).map(|i| count_visible(i, &heights)).max().unwrap();
    write!(writer, "{}", ans).unwrap();
}

fn count_visible(i: usize, heights: &[i32]) -> usize {
    let mut cnt = 0;
    count_slope!((0..i).rev(), i, heights, -1.0, cnt);
    count_slope!(i + 1..heights.len(), i, heights, 1.0, cnt);
    cnt
}

fn calc_slope(x1: usize, y1: i32, x2: usize, y2: i32) -> f64 {
    (y1 - y2) as f64 / (x1 as f64 - x2 as f64)
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

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
