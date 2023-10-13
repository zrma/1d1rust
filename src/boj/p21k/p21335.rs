use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve21335(reader: &mut impl BufRead, writer: &mut impl Write) {
    let area = read_line(reader).parse::<f64>().unwrap();

    let r = (area / std::f64::consts::PI).sqrt();

    let ans = 2.0 * std::f64::consts::PI * r;
    write!(writer, "{:.6}", ans).unwrap();
}

// https://www.acmicpc.net/problem/21335
// Another Eruption
#[test]
fn test_solve21335() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "50".to_string(),
            want: "25.066282746".to_string(),
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
        solve21335(&mut reader, &mut writer);

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
