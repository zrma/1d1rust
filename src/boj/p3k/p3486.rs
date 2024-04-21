use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve3486(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_cases = read_value(read_line(reader));
    let mut results = Vec::new();

    for _ in 0..num_cases {
        let (a, b) = read_values_as!(read_line(reader), String, String);
        let reversed_sum = reverse_sum(&a, &b);
        results.push(reversed_sum);
    }

    write!(writer, "{}", results.join("\n")).expect("Failed to write");
}

fn reverse_sum(a: &str, b: &str) -> String {
    let reversed_a = reverse_and_clean(a);
    let reversed_b = reverse_and_clean(b);
    let sum = reversed_a.parse::<i64>().expect("Failed to parse a")
        + reversed_b.parse::<i64>().expect("Failed to parse b");
    reverse_and_clean(&sum.to_string())
}

fn reverse_and_clean(number: &str) -> String {
    number
        .trim_start_matches('0')
        .chars()
        .rev()
        .collect::<String>()
        .trim_start_matches('0')
        .to_string()
}

// https://www.acmicpc.net/problem/3486
// noinspection SpellCheckingInspection
// Adding Reversed Numbers
#[test]
fn test_solve3486() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
24 1
4358 754
305 794"
                .to_string(),
            want: "34
1998
1"
            .to_string(),
        },
        TestData {
            s: "3
55 64
55 74
55 65"
                .to_string(),
            want: "101
201
111"
            .to_string(),
        },
        TestData {
            s: "1
1 1000"
                .to_string(),
            want: "2".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve3486(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
