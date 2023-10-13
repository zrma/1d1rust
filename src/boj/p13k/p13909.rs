use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve13909(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_line(reader).parse::<u64>().unwrap();
    let mut ans = 0;
    for i in 1..=n {
        if i * i > n {
            break;
        }
        ans += 1;
    }
    write!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/13909
// 창문 닫기
#[test]
fn test_solve13909() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3".to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "24".to_string(),
            want: "4".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve13909(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
