use crate::utils::functions::check_palindrome_nth;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve13235(reader: &mut impl BufRead, writer: &mut impl Write) {
    let s = read_line(reader);
    let chars = s.as_bytes();

    let mut ans = true;
    for i in 0..s.len() / 2 {
        if !check_palindrome_nth(chars, i) {
            ans = false;
            break;
        }
    }

    write!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/13235
// 팰린드롬
// noinspection SpellCheckingInspection
#[test]
fn test_solve13235() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "a".to_string(),
            want: "true".to_string(),
        },
        TestData {
            s: "uu".to_string(),
            want: "true".to_string(),
        },
        TestData {
            s: "owo".to_string(),
            want: "true".to_string(),
        },
        TestData {
            s: "bbbbbbbbbbb".to_string(),
            want: "true".to_string(),
        },
        TestData {
            s: "zzzzzzzzo".to_string(),
            want: "false".to_string(),
        },
        TestData {
            s: "slakdjfims".to_string(),
            want: "false".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve13235(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "Failed test case {}", i);
    }
}
