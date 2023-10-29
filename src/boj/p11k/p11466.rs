use crate::read_values;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve11466(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (mut h, mut w) = read_values!(read_line(reader), f64, f64);

    if h < w {
        std::mem::swap(&mut h, &mut w);
    }

    let ans = if h >= w * 1.5 {
        min(w, h / 3.0)
    } else {
        w / 2.0
    };

    write!(writer, "{}", ans).unwrap();
}

fn min(a: f64, b: f64) -> f64 {
    if a < b {
        a
    } else {
        b
    }
}

// https://www.acmicpc.net/problem/11466
// Alex Origami Squares
#[test]
fn test_solve11466() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "210 297".to_string(),
            want: "105.0".to_string(),
        },
        TestData {
            s: "250 100".to_string(),
            want: "83.333333".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve11466(&mut reader, &mut writer);

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
