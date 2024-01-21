use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve5357(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_cases: usize = read_value(read_line(reader));
    let mut s = String::new();
    let mut ans = Vec::with_capacity(num_cases);

    for _ in 0..num_cases {
        s.clear();
        reader.read_line(&mut s).unwrap();
        ans.push(deduplicate(&s));
    }

    write!(writer, "{}", ans.join("\n")).unwrap();
}

fn deduplicate(s: &str) -> String {
    s.trim_end()
        .chars()
        .fold(Vec::new(), |mut acc, ch| {
            if acc.last() != Some(&ch) {
                acc.push(ch);
            }
            acc
        })
        .iter()
        .collect()
}

// https://www.acmicpc.net/problem/5357
// noinspection SpellCheckingInspection
// Dedupe
#[test]
fn test1_5357() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
ABBBBAACC
AAAAA
ABC"
            .to_string(),
            want: "ABAC
A
ABC"
            .to_string(),
        },
        TestData {
            s: "1
A"
            .to_string(),
            want: "A".to_string(),
        },
        TestData {
            s: "1
AAAAAAAAAAAAAAAAAAAAA"
                .to_string(),
            want: "A".to_string(),
        },
        TestData {
            s: "1
AAAAAAAAAAAAAAAAAAAAAB"
                .to_string(),
            want: "AB".to_string(),
        },
        TestData {
            s: "1
AAAAAAAAAAAAAAAAAAAAABBA"
                .to_string(),
            want: "ABA".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve5357(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
