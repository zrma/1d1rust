use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve16478(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (pab, pbc, pcd) = read_values_as!(read_line(reader), f64, f64, f64);

    let ans = pab * pcd / pbc;
    write!(writer, "{}", ans).expect("Failed to write");
}

// https://www.acmicpc.net/problem/16478
// 원의 분할
#[test]
fn test_solve16478() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5 5 5".to_string(),
            want: "5".to_string(),
        },
        TestData {
            s: "9 8 13".to_string(),
            want: "14.625".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve16478(&mut reader, &mut writer);

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
