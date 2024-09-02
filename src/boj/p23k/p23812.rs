use crate::boj::p23k::p23811::append_pattern;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve23812(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));

    let odd_line = format!("{}{}{}", "@".repeat(n), " ".repeat(n * 3), "@".repeat(n));
    let even_line = "@".repeat(5 * n);

    let mut ans = String::with_capacity(5 * n * 5 * n);

    append_pattern(&mut ans, n, &even_line, &odd_line);

    write!(writer, "{}", ans).expect("Failed to write");
}

// https://www.acmicpc.net/problem/23812
// 골뱅이 찍기 - 돌아간 ㅍ
#[test]
fn test_solve23812() {
    struct TestData {
        s: String,
        want: String,
    }

    for (i, data) in [
        TestData {
            s: "1".to_string(),
            want: "@   @
@@@@@
@   @
@@@@@
@   @"
                .to_string(),
        },
        TestData {
            s: "3".to_string(),
            want: "@@@         @@@
@@@         @@@
@@@         @@@
@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@
@@@         @@@
@@@         @@@
@@@         @@@
@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@
@@@         @@@
@@@         @@@
@@@         @@@"
                .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve23812(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
