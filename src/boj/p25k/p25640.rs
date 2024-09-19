use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve25640(reader: &mut impl BufRead, writer: &mut impl Write) {
    let s = read_line(reader);
    let n: usize = read_line(reader).parse().unwrap();

    let mut ans = 0;
    for _ in 0..n {
        let t = read_line(reader);
        if s == t {
            ans += 1;
        }
    }

    write!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/25640
// MBTI
// noinspection SpellCheckingInspection
#[test]
fn test_solve25640() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "ESTJ
5
ISTP
ESTJ
INTP
ESTJ
ENTJ"
                .to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "INTP
6
INTP
INTP
ESFP
ISFP
INFP
INTP"
                .to_string(),
            want: "3".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve25640(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
