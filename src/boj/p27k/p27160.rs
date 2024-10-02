use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::collections::HashMap;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve27160(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_value(read_line(reader));

    let mut counts = HashMap::new();
    for _ in 0..n {
        let (name, count) = read_values_as!(read_line(reader), String, u32);
        *counts.entry(name).or_insert(0) += count;
    }

    let ans = counts.values().any(|&count| count == 5);
    write!(writer, "{}", if ans { "YES" } else { "NO" }).unwrap();
}

// https://www.acmicpc.net/problem/27160
// noinspection SpellCheckingInspection
// 할리갈리
#[test]
fn test_solve27160() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
BANANA 2
PLUM 4
BANANA 3"
                .to_string(),
            want: "YES".to_string(),
        },
        TestData {
            s: "4
STRAWBERRY 1
BANANA 2
LIME 3
PLUM 4"
                .to_string(),
            want: "NO".to_string(),
        },
        TestData {
            s: "2
LIME 5
LIME 1"
                .to_string(),
            want: "NO".to_string(),
        },
        TestData {
            s: "2
BANANA 5
BANANA 5"
                .to_string(),
            want: "NO".to_string(),
        },
        TestData {
            s: "1
BANANA 5"
                .to_string(),
            want: "YES".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve27160(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
