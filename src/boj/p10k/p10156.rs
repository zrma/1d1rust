use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve10156(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (k, n, m) = read_values_as!(read_line(reader), i32, i32, i32);

    let result = (k * n - m).max(0);
    writeln!(writer, "{}", result).unwrap();
}

// https://www.acmicpc.net/problem/10156
// 과자
#[test]
fn test_solve10156() {
    struct TestCase {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestCase {
            s: "300 4 1000".to_string(),
            want: "200".to_string(),
        },
        TestCase {
            s: "250 2 140".to_string(),
            want: "360".to_string(),
        },
        TestCase {
            s: "20 6 120".to_string(),
            want: "0".to_string(),
        },
        TestCase {
            s: "20 10 320".to_string(),
            want: "0".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve10156(&mut reader, &mut writer);

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
