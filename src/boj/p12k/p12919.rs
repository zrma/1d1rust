use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve12919(reader: &mut impl BufRead, writer: &mut impl Write) {
    let s = read_line(reader);
    let t = read_line(reader);

    let res = can_convert(&s, &t);

    write!(writer, "{}", if res { 1 } else { 0 }).unwrap();
}

fn can_convert(s: &str, t: &str) -> bool {
    if s.len() == t.len() {
        return s == t;
    }

    if t.ends_with('A') && can_convert(s, &t[..t.len() - 1]) {
        return true;
    }
    if t.starts_with('B') && can_convert(s, &t[1..].chars().rev().collect::<String>()) {
        return true;
    }
    false
}

// https://www.acmicpc.net/problem/12919
// A와 B 2
// noinspection SpellCheckingInspection
#[test]
fn test_solve12919() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestData {
            s: "A
BABA"
                .to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "BAAAAABAA
BAABAAAAAB"
                .to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "A
ABBA"
                .to_string(),
            want: "0".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        use std::io::Cursor;
        let mut reader = Cursor::new(&data.s);
        let mut writer = Cursor::new(Vec::new());
        solve12919(&mut reader, &mut writer);

        let res = String::from_utf8(writer.into_inner()).unwrap();
        assert_eq!(res, data.want, "failed at {}th try", i);
    }
}
