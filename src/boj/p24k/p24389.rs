use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve24389(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: i32 = read_value(read_line(reader));
    let two_complement = (!n) + 1;
    let diff = n ^ two_complement;
    let count = diff.count_ones();
    writeln!(writer, "{}", count).unwrap();
}

// https://www.acmicpc.net/problem/24389
// 2의 보수
#[test]
fn test_solve24389() {
    struct TestCase {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestCase {
            s: "1".to_string(),
            want: "31".to_string(),
        },
        TestCase {
            s: "2".to_string(),
            want: "30".to_string(),
        },
        TestCase {
            s: "3".to_string(),
            want: "31".to_string(),
        },
        TestCase {
            s: "4".to_string(),
            want: "29".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve24389(&mut reader, &mut writer);

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
