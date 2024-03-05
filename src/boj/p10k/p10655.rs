use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve10655(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_value(read_line(reader));

    let mut points = vec![];
    for _ in 0..n {
        let (x, y) = read_values_as!(read_line(reader), i32, i32);
        points.push((x, y));
    }

    let mut dist = 0;
    for i in 1..n {
        dist += manhattan_dist(points[i - 1], points[i]);
    }

    let mut max_skip_dist = 0;
    for i in 1..n - 1 {
        let skip_dist = manhattan_dist(points[i - 1], points[i + 1]);
        let normal_dist =
            manhattan_dist(points[i - 1], points[i]) + manhattan_dist(points[i], points[i + 1]);
        max_skip_dist = max_skip_dist.max(normal_dist - skip_dist);
    }

    let res = dist - max_skip_dist;
    write!(writer, "{}", res).expect("Failed to write");
}

type Point = (i32, i32);

fn manhattan_dist(p1: Point, p2: Point) -> i32 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

// https://www.acmicpc.net/problem/10655
// 마라톤 1
#[test]
fn test_solve10655() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "4
0 0
8 3
11 -1
10 0"
                .to_string(),
            want: "14".to_string(),
        },
        TestData {
            s: "3
0 0
8 3
10 0"
                .to_string(),
            want: "10".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve10655(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
