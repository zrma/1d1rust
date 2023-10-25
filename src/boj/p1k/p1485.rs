use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1485(reader: &mut impl BufRead, writer: &mut impl Write) {
    let t = read_line(reader).parse::<usize>().unwrap();
    for _ in 0..t {
        let mut points = vec![];
        for _ in 0..4 {
            let s = read_line(reader);
            let mut iter = s.split_whitespace();
            let x = iter.next().unwrap().parse::<i32>().unwrap();
            let y = iter.next().unwrap().parse::<i32>().unwrap();
            points.push((x, y));
        }

        points.sort();

        let is_square = {
            let d1 = dist(points[0], points[1]);
            let d2 = dist(points[0], points[2]);
            let d3 = dist(points[1], points[3]);
            let d4 = dist(points[2], points[3]);

            let d5 = dist(points[0], points[3]);
            let d6 = dist(points[1], points[2]);

            d1 == d2 && d2 == d3 && d3 == d4 && d4 == d1 && d5 == d6
        };

        if is_square {
            writeln!(writer, "1").unwrap();
        } else {
            writeln!(writer, "0").unwrap();
        }
    }
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
