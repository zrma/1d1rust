use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve15964(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (a, b): (i64, i64) = read_values_as!(read_line(reader), i64, i64);
    let result = (a + b) * (a - b);
    writeln!(writer, "{}", result).unwrap();
}

// https://www.acmicpc.net/problem/15964
// 이상한 기호
#[test]
fn test_solve15964() {
    struct TestCase {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestCase {
            s: "4 3".to_string(),
            want: "7".to_string(),
        },
        TestCase {
            s: "3 4".to_string(),
            want: "-7".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve15964(&mut reader, &mut writer);

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
