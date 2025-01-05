use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1198(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_value(read_line(reader));

    let mut points = Vec::with_capacity(n);
    for _ in 0..n {
        points.push(read_values_as!(read_line(reader), i32, i32));
    }

    let mut max_area = 0;
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                let area = calc_area(points[i], points[j], points[k]);
                max_area = max_area.max(area);
            }
        }
    }

    let max_area_f64 = f64::from(max_area) / 2.0;

    writeln!(writer, "{}", max_area_f64).unwrap();
}

fn calc_area(p0: (i32, i32), p1: (i32, i32), p2: (i32, i32)) -> i32 {
    let (x0, y0) = p0;
    let (x1, y1) = p1;
    let (x2, y2) = p2;

    // Shoelace formula
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

        let got: f64 = read_value(String::from_utf8(writer).unwrap());
        let want: f64 = data.want.parse().unwrap();

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
