use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2965(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (a, b, c) = read_values_as!(read_line(reader), i32, i32, i32);

    let ans = (b - a).max(c - b) - 1;
    write!(writer, "{}", ans).expect("write! should work");
}

// https://www.acmicpc.net/problem/2965
// 캥거루 세마리
#[test]
fn test_solve2965() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "2 3 5".to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "3 5 9".to_string(),
            want: "3".to_string(),
        },
        TestData {
            s: "3 5 7".to_string(),
            want: "1".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve2965(&mut reader, &mut writer);

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
