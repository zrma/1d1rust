use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2783(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (x, y) = read_values_as!(read_line(reader), f64, f64);
    let n: usize = read_value(read_line(reader));

    let min_price = (0..n)
        .map(|_| {
            let (a, b) = read_values_as!(read_line(reader), f64, f64);
            a / b
        })
        .reduce(|a, b| a.min(b))
        .unwrap_or(x / y);

    let ans = min_price * 1000.0;
    write!(writer, "{:.2}", ans).unwrap();
}

// https://www.acmicpc.net/problem/2783
// 삼각 김밥
#[test]
fn test_solve2783() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5 100
3
4 100
3 100
7 100"
                .to_string(),
            want: "30.00".to_string(),
        },
        TestData {
            s: "13 6
5
56 679
35 120
99 999
56 73
37 532"
                .to_string(),
            want: "69.55".to_string(),
        },
        TestData {
            s: "100 5
3
99 8
65 14
78 10"
                .to_string(),
            want: "4642.86".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve2783(&mut reader, &mut writer);

        let got: f64 = read_value(String::from_utf8(writer).expect(
            "writer should be a valid string
",
        ));
        let want: f64 = data.want.parse().expect("data.want should be a valid f64");

        const EPSILON: f64 = 1e-2;

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
