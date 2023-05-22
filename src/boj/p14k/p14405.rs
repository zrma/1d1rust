use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve14405(reader: &mut impl BufRead, writer: &mut impl Write) {
    let s = read_line(reader);

    let mut i = 0;
    while i < s.len() {
        if s[i..].starts_with("pi") || s[i..].starts_with("ka") {
            i += 2;
        } else if s[i..].starts_with("chu") {
            i += 3;
        } else {
            write!(writer, "NO").unwrap();
            return;
        }
    }

    write!(writer, "YES").unwrap();
}

// https://www.acmicpc.net/problem/14405
// 피카츄
// noinspection SpellCheckingInspection
#[test]
fn test_14405() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestData {
            s: "pi".to_string(),
            want: "YES".to_string(),
        },
        TestData {
            s: "ka".to_string(),
            want: "YES".to_string(),
        },
        TestData {
            s: "chu".to_string(),
            want: "YES".to_string(),
        },
        TestData {
            s: "pikachu".to_string(),
            want: "YES".to_string(),
        },
        TestData {
            s: "pikapi".to_string(),
            want: "YES".to_string(),
        },
        TestData {
            s: "pipikachu".to_string(),
            want: "YES".to_string(),
        },
        TestData {
            s: "pikaqiu".to_string(),
            want: "NO".to_string(),
        },
        TestData {
            s: "piika".to_string(),
            want: "NO".to_string(),
        },
        TestData {
            s: "pikachuq".to_string(),
            want: "NO".to_string(),
        },
        TestData {
            s: "pikachuchu".to_string(),
            want: "YES".to_string(),
        },
        TestData {
            s: "chupikachupipichu".to_string(),
            want: "YES".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        use std::io::Cursor;
        let mut reader = Cursor::new(&data.s);
        let mut writer = Cursor::new(Vec::new());
        solve14405(&mut reader, &mut writer);

        let got = String::from_utf8(writer.into_inner()).unwrap();
        assert_eq!(got, data.want, "case {}", i);
    }
}
