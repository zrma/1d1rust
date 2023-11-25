use crate::read_values;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve6600(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut line = String::new();
    while reader.read_line(&mut line).is_ok() && !line.is_empty() {
        let (x1, y1, x2, y2, x3, y3) = read_values!(&line, f64, f64, f64, f64, f64, f64);
        let circumference = 2.0 * std::f64::consts::PI * calc_radius(x1, y1, x2, y2, x3, y3);
        writeln!(writer, "{:.2}", circumference).unwrap();
        line.clear();
    }
}

fn calc_radius(x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) -> f64 {
    let a = calc_dist(x1, y1, x2, y2);
    let b = calc_dist(x2, y2, x3, y3);
    let c = calc_dist(x3, y3, x1, y1);
    let s = (a + b + c) / 2.0;
    let area = (s * (s - a) * (s - b) * (s - c)).sqrt();
    a * b * c / (4.0 * area)
}

fn calc_dist(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt()
}

// https://www.acmicpc.net/problem/6600
// 원의 둘레
#[test]
fn test_solve6600() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "0.0 -0.5 0.5 0.0 0.0 0.5
0.0 0.0 0.0 1.0 1.0 1.0
5.0 5.0 5.0 7.0 4.0 6.0
0.0 0.0 -1.0 7.0 7.0 7.0
50.0 50.0 50.0 70.0 40.0 60.0
0.0 0.0 10.0 0.0 20.0 1.0
0.0 -500000.0 500000.0 0.0 0.0 500000.0"
                .to_string(),
            want: "3.14
4.44
6.28
31.42
62.83
632.24
3141592.65"
                .to_string(),
        },
        TestData {
            s: "0.0 0.0 -1.0 7.0 7.0 7.0
50.0 50.0 50.0 70.0 40.0 60.0"
                .to_string(),
            want: "31.42
62.83"
                .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve6600(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        let want = data.want.split_whitespace();

        for (j, (got, want)) in got.split_whitespace().zip(want).enumerate() {
            let got = got.parse::<f64>().unwrap();
            let want = want.parse::<f64>().unwrap();

            const EPSILON: f64 = 1e-6;

            let abs_diff = (got - want).abs();
            let rel_diff = abs_diff / want.abs().max(1e-9);

            assert!(
                abs_diff < EPSILON || rel_diff < EPSILON,
                "case {}-{}: absolute error: {}, relative error: {}",
                i,
                j,
                abs_diff,
                rel_diff
            );
        }
    }
}
