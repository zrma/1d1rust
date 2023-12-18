use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve20944(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_value(read_line(reader));
    write!(writer, "{}", "a".repeat(n)).unwrap();
}

// https://www.acmicpc.net/problem/20944
// noinspection SpellCheckingInspection
// 팰린드롬 척화비
#[test]
fn test_solve20944() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "7".to_string(),
            want: "aaaaaaa".to_string(),
        },
        TestData {
            s: "1".to_string(),
            want: "a".to_string(),
        },
        TestData {
            s: "2".to_string(),
            want: "aa".to_string(),
        },
        TestData {
            s: "3".to_string(),
            want: "aaa".to_string(),
        },
        TestData {
            s: "4".to_string(),
            want: "aaaa".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve20944(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
