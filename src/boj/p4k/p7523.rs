use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve7523(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_cases: usize = read_value(read_line(reader));

    let ans: String = (0..num_cases)
        .map(|i| {
            let (a, b): (i64, i64) = read_values_as!(read_line(reader), i64, i64);
            let n = b - a + 1;
            let sum = (a + b) * n / 2;

            format!("Scenario #{}:\n{}\n", i + 1, sum)
        })
        .collect::<Vec<_>>()
        .join("\n");

    write!(writer, "{}", ans).expect("write! should work");
}

// https://www.acmicpc.net/problem/7523
// Gauß
#[test]
fn test_solve7523() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
1 100
-11 10
-89173 938749341"
                .to_string(),
            want: "Scenario #1:
5050

Scenario #2:
-11

Scenario #3:
440625159107385260
"
            .to_string(),
        },
        TestData {
            s: "1
            1 10"
                .to_string(),
            want: "Scenario #1:
55
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve7523(&mut reader, &mut writer);

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
