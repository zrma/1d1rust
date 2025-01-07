use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve8595(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_line(reader).parse().unwrap();
    let s = read_line(reader);

    let s = &s[..n];

    let mut sum: u64 = 0;
    let mut num_str = String::new();

    for ch in s.chars() {
        if ch.is_numeric() {
            num_str.push(ch);
        } else if !num_str.is_empty() {
            sum += num_str.parse().unwrap_or(0);
            num_str.clear();
        }
    }

    // Check for a number at the end of the string
    if !num_str.is_empty() {
        sum += num_str.parse().unwrap_or(0);
    }

    writeln!(writer, "{}", sum).unwrap();
}

// https://www.acmicpc.net/problem/8595
// 히든 넘버
// noinspection SpellCheckingInspection
#[test]
fn test_solve8595() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "14
ab13c9d07jeden"
                .to_string(),
            want: "29".to_string(),
        },
        TestData {
            s: "6
12ab27"
                .to_string(),
            want: "39".to_string(),
        },
        TestData {
            s: "4
abcd"
                .to_string(),
            want: "0".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve8595(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(
            got.trim(),
            data.want.trim(),
            "failed at {} with {}",
            i,
            data.s
        );
    }
}
