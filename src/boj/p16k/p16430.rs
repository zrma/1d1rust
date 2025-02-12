use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve16430(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (a, b): (i32, i32) = read_values_as!(read_line(reader), i32, i32);
    let p = b - a;
    let q = b;
    writeln!(writer, "{} {}", p, q).unwrap();
}

// https://www.acmicpc.net/problem/16430
// 제리와 톰
#[test]
fn test_solve16430() {
    struct TestCase {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestCase {
            s: "1 3".to_string(),
            want: "2 3".to_string(),
        },
        TestCase {
            s: "2 7".to_string(),
            want: "5 7".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve16430(&mut reader, &mut writer);

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
