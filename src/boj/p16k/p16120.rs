use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
// noinspection SpellCheckingInspection
fn solve16120(reader: &mut impl BufRead, writer: &mut impl Write) {
    let s = read_line(reader);

    let mut stack = Vec::new();
    for c in s.chars() {
        stack.push(c);

        while stack.len() >= 4 && stack[stack.len() - 4..] == ['P', 'P', 'A', 'P'] {
            stack.pop();
            stack.pop();
            stack.pop();
        }
    }

    if stack == ['P'] {
        write!(writer, "PPAP").unwrap();
    } else {
        write!(writer, "NP").unwrap();
    }
}

// https://www.acmicpc.net/problem/16120
// noinspection SpellCheckingInspection
// PPAP
#[test]
fn test_16120() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestData {
            s: "PPAP".to_string(),
            want: "PPAP".to_string(),
        },
        TestData {
            s: "PPPAPAP".to_string(),
            want: "PPAP".to_string(),
        },
        TestData {
            s: "PPAPAPP".to_string(),
            want: "NP".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        use std::io::Cursor;
        let mut reader = Cursor::new(&data.s);
        let mut writer = Cursor::new(Vec::new());
        solve16120(&mut reader, &mut writer);

        let got = String::from_utf8(writer.into_inner()).unwrap();
        assert_eq!(data.want, got, "case {}", i);
    }
}
