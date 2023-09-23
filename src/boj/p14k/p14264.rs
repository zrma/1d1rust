use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve14264(reader: &mut impl BufRead, writer: &mut impl Write) {
    let f = read_line(reader).parse::<f64>().unwrap();
    let ans = 3f64.sqrt() / 4.0 * f * f;
    write!(writer, "{:e}", ans).unwrap();
}

// https://www.acmicpc.net/problem/14264
// 정육각형과 삼각형
#[test]
fn test_solve14264() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestData {
            s: "5".to_string(),
            want: "10.825317547305485".to_string(),
        },
        TestData {
            s: "10".to_string(),
            want: "43.30127018922194".to_string(),
        },
        TestData {
            s: "100000".to_string(),
            want: "4.330127018922194E9".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve14264(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap().parse::<f64>().unwrap();
        let want = data.want.parse::<f64>().unwrap();

        const EPSILON: f64 = 1e-9;

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
