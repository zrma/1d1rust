use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1198(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_line(reader).parse::<usize>().unwrap();

    let mut points = vec![];
    for _ in 0..n {
        let s = read_line(reader);
        let mut iter = s.split_whitespace();
        let (x, y) = (
            iter.next().unwrap().parse::<i32>().unwrap(),
            iter.next().unwrap().parse::<i32>().unwrap(),
        );
        points.push((x, y));
    }

    let mut max_area = 0;
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                let area = calc_area(
                    points[i].0,
                    points[i].1,
                    points[j].0,
                    points[j].1,
                    points[k].0,
                    points[k].1,
                );
                max_area = max_area.max(area);
            }
        }
    }

    write!(writer, "{}", max_area as f64 / 2.0).unwrap();
}

fn calc_area(x0: i32, y0: i32, x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    // https://en.wikipedia.org/wiki/Shoelace_formula
    (x0 * y1 + x1 * y2 + x2 * y0 - x1 * y0 - x2 * y1 - x0 * y2).abs()
}

// https://www.acmicpc.net/problem/1198
// 삼각형으로 자르기
#[test]
fn test_solve1198() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
1 1
2 3
3 2"
            .to_string(),
            want: "1.5".to_string(),
        },
        TestData {
            s: "4
1 1
1 2
3 3
2 1"
            .to_string(),
            want: "1.5".to_string(),
        },
        TestData {
            s: "8
1 2
1 3
2 4
3 4
4 3
4 2
3 1
2 1"
            .to_string(),
            want: "3.0".to_string(),
        },
        TestData {
            s: "7
6 2
2 1
1 2
1 4
2 6
5 6
6 5"
            .to_string(),
            want: "10.0".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve1198(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap().parse::<f64>().unwrap();
        let want = data.want.parse::<f64>().unwrap();

        const EPSILON: f64 = 1e-6;

        let abs_diff = (got - want).abs();
        let rel_diff = abs_diff / want.abs().max(1e-9);

        assert!(
            abs_diff < EPSILON || rel_diff < EPSILON,
            "case {}: absolute error: {}, relative error: {}",
            i,
            abs_diff,
            rel_diff
        );
    }
}
