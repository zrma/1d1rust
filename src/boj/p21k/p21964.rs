use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve21964(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_line(reader).parse().unwrap();
    let s = read_line(reader);

    let ans: String = s.chars().skip(n - 5).take(5).collect::<_>();
    write!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/21964
// noinspection SpellCheckingInspection
// 선린인터넷고등학교 교가
#[test]
fn test_solve21964() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "12
Sunrin,Hair."
                .to_string(),
            want: "Hair.".to_string(),
        },
        TestData {
            s: "5
Sunrin,High."
                .to_string(),
            want: "Sunri".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve21964(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
