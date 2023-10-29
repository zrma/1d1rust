use crate::read_values;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1485(reader: &mut impl BufRead, writer: &mut impl Write) {
    let t = read_line(reader).parse::<usize>().unwrap();
    for _ in 0..t {
        let points = (0..4)
            .map(|_| read_values!(read_line(reader), i32, i32))
            .collect::<Vec<_>>();

        if is_square(&points) {
            writeln!(writer, "1").unwrap();
        } else {
            writeln!(writer, "0").unwrap();
        }
    }
}

fn is_square(points: &[(i32, i32)]) -> bool {
    let mut sorted_points = points.to_vec();
    sorted_points.sort();

    let d1 = dist(sorted_points[0], sorted_points[1]);
    let d2 = dist(sorted_points[0], sorted_points[2]);
    let d3 = dist(sorted_points[1], sorted_points[3]);
    let d4 = dist(sorted_points[2], sorted_points[3]);

    let d5 = dist(sorted_points[0], sorted_points[3]);
    let d6 = dist(sorted_points[1], sorted_points[2]);

    d1 == d2 && d2 == d3 && d3 == d4 && d4 == d1 && d5 == d6
}

fn dist(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    (p1.0 - p2.0).pow(2) + (p1.1 - p2.1).pow(2)
}

// https://www.acmicpc.net/problem/1485
// 정사각형
#[test]
fn test_solve1485() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "2
1 1
1 2
2 1
2 2
2 2
3 3
4 4
5 5"
            .to_string(),
            want: "1
0
"
            .to_string(),
        },
        TestData {
            s: "1
1 1
1 2
2 1
2 2"
            .to_string(),
            want: "1
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve1485(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
