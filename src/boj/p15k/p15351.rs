use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve15351(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_value(read_line(reader));
    let ans = (0..n)
        .map(|_| read_line(reader))
        .map(compute_name_value)
        .map(format_result)
        .collect::<Vec<_>>();

    write!(writer, "{}", ans.join("\n")).unwrap();
}

fn compute_name_value(name: String) -> usize {
    name.chars()
        .filter(|&c| c.is_ascii_alphabetic())
        .map(|c| (c.to_ascii_uppercase() as usize) - ('A' as usize) + 1)
        .sum()
}

fn format_result(value: usize) -> String {
    match value {
        100 => "PERFECT LIFE".to_string(),
        _ => value.to_string(),
    }
}

// https://www.acmicpc.net/problem/15351
// noinspection SpellCheckingInspection
// 인생 점수
#[test]
fn test_solve15351() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "4
OTAKU LIFE
PRODUCER
GAMING LIFE
PROGRAMMING"
                .to_string(),
            want: "PERFECT LIFE
PERFECT LIFE
83
131"
            .to_string(),
        },
        TestData {
            s: "2
I AM NOT SMART
ACM IS MY LIFE"
                .to_string(),
            want: "143
115"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve15351(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
