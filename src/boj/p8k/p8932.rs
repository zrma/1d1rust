use crate::utils::io::{read_line, read_n_values, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve8932(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_cases: usize = read_value(read_line(reader));

    enum S {
        Track,
        Field,
    }

    struct Score {
        a: f64,
        b: f64,
        c: f64,
        s: S,
    }
    let scores = [
        Score {
            a: 9.23076,
            b: 26.7,
            c: 1.835,
            s: S::Track,
        },
        Score {
            a: 1.84523,
            b: 75.0,
            c: 1.348,
            s: S::Field,
        },
        Score {
            a: 56.0211,
            b: 1.5,
            c: 1.05,
            s: S::Field,
        },
        Score {
            a: 4.99087,
            b: 42.5,
            c: 1.81,
            s: S::Track,
        },
        Score {
            a: 0.188807,
            b: 210.0,
            c: 1.41,
            s: S::Field,
        },
        Score {
            a: 15.9803,
            b: 3.8,
            c: 1.04,
            s: S::Field,
        },
        Score {
            a: 0.11193,
            b: 254.0,
            c: 1.88,
            s: S::Track,
        },
    ];

    let mut res = Vec::with_capacity(num_cases);
    for _ in 0..num_cases {
        let values: Vec<f64> = read_n_values(reader, 7);
        let sum = values.iter().zip(scores.iter()).fold(0, |acc, (v, s)| {
            let p = *v;
            let score = match s.s {
                S::Track => s.a * ((s.b - p).powf(s.c)),
                S::Field => s.a * ((p - s.b).powf(s.c)),
            };
            acc + score as i32
        });

        res.push(sum.to_string());
    }

    write!(writer, "{}", res.join("\n")).expect("Failed to write");
}

// https://www.acmicpc.net/problem/8932
// 7종 경기
#[test]
fn test_solve8932() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
19 90 11 29 264 20 131
12 95 21 37 224 35 221
17 168 15 23 275 22 241"
                .to_string(),
            want: "2901
3419
3772"
                .to_string(),
        },
        TestData {
            s: "1
26 75 2 42 210 4 254"
                .to_string(),
            want: "34".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve8932(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(
            got.trim(),
            data.want.trim(),
            "failed at {} with {}",
            i,
            data.s
        );
    }
}
