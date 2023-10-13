use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve16171(reader: &mut impl BufRead, writer: &mut impl Write) {
    let s = read_line(reader);
    let k = read_line(reader);

    let s = s
        .chars()
        .filter(|c| !c.is_ascii_digit())
        .collect::<String>();
    let ans = if s.contains(&k) { 1 } else { 0 };

    write!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/16171
// 나는 친구가 적다 (Small)
#[test]
fn test_solve16171() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "1q2w3e4r5t6y
qwerty"
                .to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "1ovey0uS2
veS"
            .to_string(),
            want: "0".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve16171(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "Failed test case {}", i);
    }
}
