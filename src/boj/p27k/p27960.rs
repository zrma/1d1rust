use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve27960(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (a, b) = read_values_as!(read_line(reader), usize, usize);
    let ans = a ^ b;
    writeln!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/27960
// 사격 내기
#[test]
fn test_solve27960() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "55 73".to_string(),
            want: "126".to_string(),
        },
        TestData {
            s: "105 47".to_string(),
            want: "70".to_string(),
        },
        TestData {
            s: "197 197".to_string(),
            want: "0".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve27960(&mut reader, &mut writer);

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
