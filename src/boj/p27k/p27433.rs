use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve27433(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_line(reader).parse::<u64>().unwrap();
    let mut ans = 1;
    for i in 1..=n {
        ans *= i;
    }
    write!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/27433
// 팩토리얼 2
#[test]
fn test_solve27433() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "10".to_string(),
            want: "3628800".to_string(),
        },
        TestData {
            s: "0".to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "1".to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "2".to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "3".to_string(),
            want: "6".to_string(),
        },
        TestData {
            s: "4".to_string(),
            want: "24".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve27433(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
