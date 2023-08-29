use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve15813(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_line(reader).parse::<usize>().unwrap();
    let s = read_line(reader);

    let mut ans = 0;
    for i in 0..n {
        ans += s.chars().nth(i).unwrap() as usize - 'A' as usize + 1;
    }

    write!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/15813
// 너의 이름은 몇 점이니?
#[test]
fn test_solve15813() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in vec![
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

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}