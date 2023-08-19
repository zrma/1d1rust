use crate::utils::functions::check_palindrome_nth;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve15927(reader: &mut impl BufRead, writer: &mut impl Write) {
    let s = read_line(reader);
    let chars = s.chars().collect::<Vec<_>>();

    let mut is_palindrome = true;
    let mut is_all_same = true;

    let first = s.chars().next().unwrap();

    for i in 0..s.len() / 2 + 1 {
        if !check_palindrome_nth(&chars, i) {
            is_palindrome = false;
        }
        if chars[i] != first {
            is_all_same = false;
        }
    }

    if is_palindrome {
        if is_all_same {
            write!(writer, "-1").unwrap();
        } else {
            write!(writer, "{}", s.len() - 1).unwrap();
        }
    } else {
        write!(writer, "{}", s.len()).unwrap();
    }
}

// https://www.acmicpc.net/problem/15927
// 회문은 회문아니야!!
// noinspection SpellCheckingInspection
#[test]
fn test_solve15927() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestData {
            s: "ABCBA".to_string(),
            want: "4".to_string(),
        },
        TestData {
            s: "PALINDROME".to_string(),
            want: "10".to_string(),
        },
        TestData {
            s: "ZZZ".to_string(),
            want: "-1".to_string(),
        },
        TestData {
            s: "ABCBACBCA".to_string(),
            want: "9".to_string(),
        },
        TestData {
            s: "AABAA".to_string(),
            want: "4".to_string(),
        },
        TestData {
            s: "AABBAA".to_string(),
            want: "5".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve15927(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "case {}", i);
    }
}
