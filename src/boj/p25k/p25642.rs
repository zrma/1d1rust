use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve25642(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (mut yt, mut yj) = read_values_as!(read_line(reader), u8, u8);

    let mut turn = 0;
    while yt < 5 && yj < 5 {
        if turn % 2 == 0 {
            yj += yt;
        } else {
            yt += yj;
        }
        turn += 1;
    }

    let ans = if yt >= 5 { "yj" } else { "yt" };
    write!(writer, "{}", ans).expect("Failed to write");
}

// https://www.acmicpc.net/problem/25642
// 젓가락 게임
#[test]
fn test_solve25642() {
    struct TestData {
        s: String,
        want: String,
    }

    for (i, data) in [
        TestData {
            s: "1 2".to_string(),
            want: "yt".to_string(),
        },
        TestData {
            s: "2 2".to_string(),
            want: "yj".to_string(),
        },
        TestData {
            s: "3 1".to_string(),
            want: "yj".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve25642(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
