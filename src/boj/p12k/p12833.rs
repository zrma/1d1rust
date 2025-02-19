use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve12833(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (a, b, c): (i32, i32, i32) = read_values_as!(read_line(reader), i32, i32, i32);
    let result = if c % 2 == 0 { a } else { a ^ b };
    writeln!(writer, "{}", result).unwrap();
}

// https://www.acmicpc.net/problem/12833
// XORXORXOR
#[test]
fn test_solve12833() {
    struct TestCase {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestCase {
            s: "13 3 1".to_string(),
            want: "14".to_string(),
        },
        TestCase {
            s: "2 7 3".to_string(),
            want: "5".to_string(),
        },
        TestCase {
            s: "2 7 4".to_string(),
            want: "2".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve12833(&mut reader, &mut writer);

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
