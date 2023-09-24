use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve16486(reader: &mut impl BufRead, writer: &mut impl Write) {
    let d1 = read_line(reader).parse::<f64>().unwrap();
    let d2 = read_line(reader).parse::<f64>().unwrap();

    let ans = 2.0 * d1 + 2.0 * std::f64::consts::PI * d2;
    write!(writer, "{:.6}", ans).unwrap();
}

// https://www.acmicpc.net/problem/16486
// 운동장 한 바퀴
#[test]
fn test_solve16486() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestData {
            s: "13
8"
            .to_string(),
            want: "76.265472".to_string(),
        },
        TestData {
            s: "18
8"
            .to_string(),
            want: "86.265472".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve16486(&mut reader, &mut writer);

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
