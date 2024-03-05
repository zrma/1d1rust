use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve20352(reader: &mut impl BufRead, writer: &mut impl Write) {
    let a = read_line(reader).parse::<f64>().unwrap();
    let r = (a / std::f64::consts::PI).sqrt();
    let res = 2.0 * std::f64::consts::PI * r;
    write!(writer, "{}", res).unwrap();
}

// https://www.acmicpc.net/problem/20352
// Circus
#[test]
fn test_solve20352() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "64".to_string(),
            want: "28.3592616145".to_string(),
        },
        TestData {
            s: "1234".to_string(),
            want: "124.526709336".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve20352(&mut reader, &mut writer);

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
