use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve26082(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (a, b, c): (i32, i32, i32) = read_values_as!(read_line(reader), i32, i32, i32);
    let result = b * 3 * c / a;
    writeln!(writer, "{}", result).unwrap();
}

// https://www.acmicpc.net/problem/26082
// WARBOY
#[test]
fn test_solve26082() {
    struct TestCase {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestCase {
            s: "10 100 7".to_string(),
            want: "210".to_string(),
        },
        TestCase {
            s: "10 20 30".to_string(),
            want: "180".to_string(),
        },
        TestCase {
            s: "20 30 40".to_string(),
            want: "180".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve26082(&mut reader, &mut writer);

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
