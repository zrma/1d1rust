use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve16486(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (c, b) = read_values_as!(read_line(reader), f64, f64);

    let ans = c / b;
    write!(writer, "{}", ans).expect("Failed to write");
}

// https://www.acmicpc.net/problem/16486
// 작도하자! - ②
#[test]
fn test_solve16486() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "9 3".to_string(),
            want: "3".to_string(),
        },
        TestData {
            s: "15 7".to_string(),
            want: "2.1428571428".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve16486(&mut reader, &mut writer);

        let got: f64 = crate::utils::io::read_value(
            String::from_utf8(writer).expect("Failed to convert writer to string"),
        );
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
