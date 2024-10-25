use crate::utils::io::{read_line, read_values};
use std::collections::HashSet;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve26040(reader: &mut impl BufRead, writer: &mut impl Write) {
    let s = read_line(reader);
    let replacements: HashSet<char> = read_values(reader).into_iter().collect();

    let ans: String = s
        .chars()
        .map(|c| {
            if replacements.contains(&c) {
                c.to_ascii_lowercase()
            } else {
                c
            }
        })
        .collect();

    write!(writer, "{}", ans).expect("write should work");
}

// https://www.acmicpc.net/problem/26040
// 특정 대문자를 소문자로 바꾸기
#[test]
fn test_solve26040() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "ABabC
A"
            .to_string(),
            want: "aBabC".to_string(),
        },
        TestData {
            s: "ABabC
A B D"
                .to_string(),
            want: "ababC".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = Vec::new();
        solve26040(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
