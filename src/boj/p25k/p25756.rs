use crate::utils::io::{read_line, read_n_values, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve25756(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));

    let result: String = read_n_values::<f64>(reader, n)
        .into_iter()
        .scan(0.0, |v, x| {
            *v = 1.0 - (1.0 - *v) * (1.0 - x / 100.0);
            Some(format!("{:.6}\n", *v * 100.0))
        })
        .collect::<_>();

    let ans = result.trim_end(); // 마지막 개행 제거

    write!(writer, "{}", ans).expect("write! should work");
}

// https://www.acmicpc.net/problem/25756
// 방어율 무시 계산하기
#[test]
fn test_solve25756() {
    struct TestData {
        s: String,
        want: String,
    }

    for (i, data) in [
        TestData {
            s: "5
20 20 20 20 20"
                .to_string(),
            want: "20.0
36.0
48.8
59.04
67.232"
                .to_string(),
        },
        TestData {
            s: "1
100"
            .to_string(),
            want: "100.0".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve25756(&mut reader, &mut writer);

        let got: Vec<f64> = writer
            .split(|&c| c == b'\n')
            .map(|s| {
                String::from_utf8(s.to_vec())
                    .expect(
                        "writer should be a valid string
",
                    )
                    .parse()
                    .expect("Failed to parse value")
            })
            .collect();
        let want: Vec<f64> = data.want.split('\n').map(|s| s.parse().unwrap()).collect();

        const EPSILON: f64 = 1e-6;

        assert_eq!(got.len(), want.len(), "case {}: length mismatch", i);

        for (got, want) in got.iter().zip(want.iter()) {
            let abs_diff = (got - want).abs();
            let rel_diff = abs_diff / want.abs().max(1e-6);

            assert!(
                abs_diff < EPSILON || rel_diff < EPSILON,
                "case {}: absolute error: {}, relative error: {}",
                i,
                abs_diff,
                rel_diff
            );
        }
    }
}
