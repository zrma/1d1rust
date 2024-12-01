use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve16600(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: f64 = read_value(read_line(reader));
    write!(writer, "{}", n.sqrt() * 4.0).unwrap();
}

// https://www.acmicpc.net/problem/16600
// Contemporary Art
#[test]
fn test_solve16600() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "64".to_string(),
            want: "32.0".to_string(),
        },
        TestData {
            s: "16".to_string(),
            want: "16".to_string(),
        },
        TestData {
            s: "1234".to_string(),
            want: "140.51334456".to_string(),
        },
        TestData {
            s: "100000000000000".to_string(),
            want: "40000000".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve16600(&mut reader, &mut writer);

        let got: f64 = read_value(String::from_utf8(writer).expect(
            "writer should be a valid string
",
        ));
        let want: f64 = data.want.parse().expect("data.want should be a valid f64");

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
