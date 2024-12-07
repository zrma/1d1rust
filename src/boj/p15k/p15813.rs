use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve15813(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_value(read_line(reader));
    let s = read_line(reader);
    let arr = &s.as_bytes()[0..n];

    let ans: u8 = arr.iter().map(|&x| x - b'A' + 1).sum();

    write!(writer, "{}", ans).expect("write! should work");
}

// https://www.acmicpc.net/problem/15813
// 너의 이름은 몇 점이니?
#[test]
fn test_solve15813() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "7
SUNGPIL"
                .to_string(),
            want: "98".to_string(),
        },
        TestData {
            s: "6
SOYOON"
                .to_string(),
            want: "103".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve15813(&mut reader, &mut writer);

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
