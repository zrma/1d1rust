use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve31495(reader: &mut impl BufRead, writer: &mut impl Write) {
    let s = read_line(reader);
    if s.len() < 3 {
        write!(writer, "CE").expect("Failed to write");
        return;
    }

    match (s.starts_with('"'), s.ends_with('"')) {
        (true, true) => {
            write!(writer, "{}", &s[1..s.len() - 1]).expect("Failed to write");
        }
        _ => {
            write!(writer, "CE").expect("Failed to write");
        }
    }
}

// https://www.acmicpc.net/problem/31495
// noinspection SpellCheckingInspection
// 그게 무슨 코드니..
#[test]
fn test_solve31495() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "\"Hello World".to_string(),
            want: "CE".to_string(),
        },
        TestData {
            s: "\"Hello World\"".to_string(),
            want: "Hello World".to_string(),
        },
        TestData {
            s: "\"\"".to_string(),
            want: "CE".to_string(),
        },
        TestData {
            s: "\"".to_string(),
            want: "CE".to_string(),
        },
        TestData {
            s: "\"Foo".to_string(),
            want: "CE".to_string(),
        },
        TestData {
            s: "\"Foo\"".to_string(),
            want: "Foo".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve31495(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(
            got.trim(),
            data.want.trim(),
            "failed at {} with {}",
            i,
            data.s
        );
    }
}
