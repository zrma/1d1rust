use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve3512(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (n, c) = read_values_as!(read_line(reader), usize, f64);

    let (total_area, total_bedroom_area, reduced_total_area) = (0..n)
        .map(|_| read_values_as!(read_line(reader), f64, String))
        .fold((0.0, 0.0, 0.0), |(total, bedroom, reduced), (ai, ti)| {
            let total_area = total + ai;
            let total_bedroom_area = if ti == "bedroom" {
                bedroom + ai
            } else {
                bedroom
            };
            let reduced_total_area = reduced + if ti == "balcony" { ai * 0.5 } else { ai };
            (total_area, total_bedroom_area, reduced_total_area)
        });

    let ans = reduced_total_area * c;

    writeln!(writer, "{:.0}", total_area).unwrap();
    writeln!(writer, "{:.0}", total_bedroom_area).unwrap();
    writeln!(writer, "{:.6}", ans).unwrap();
}

// https://www.acmicpc.net/problem/3512
// Flat
#[test]
fn test_solve3512() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "6 75000
8 other
3 bathroom
2 bathroom
10 kitchen
16 bedroom
7 balcony"
                .to_string(),
            want: "46
16
3187500"
                .to_string(),
        },
        TestData {
            s: "2 75123
10 kitchen
15 balcony"
                .to_string(),
            want: "25
0
1314652.5"
                .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve3512(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        let want = data.want.split_whitespace();

        for (j, (got, want)) in got.split_whitespace().zip(want).enumerate() {
            if j < 2 {
                assert_eq!(got, want, "failed at {} with {}", i, data.s);
            } else {
                // The third line is float, compare with tolerance
                let got: f64 = got.parse().unwrap();
                let want: f64 = want.parse().unwrap();

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
}
