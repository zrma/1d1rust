use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve20215(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (w, h) = read_values_as!(read_line(reader), f64, f64);

    let ans = (w + h) - (w * w + h * h).sqrt();
    write!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/20215
// Cutting Corners
#[test]
fn test_solve20215() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3 4".to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "12 7".to_string(),
            want: "5.107556011".to_string(),
        },
        TestData {
            s: "1 1".to_string(),
            want: "0.585786438".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve20215(&mut reader, &mut writer);

        let got: f64 = crate::utils::io::read_value(
            String::from_utf8(writer).expect("Failed to convert writer to string"),
        );
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
