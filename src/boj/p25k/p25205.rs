use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve25205(reader: &mut impl BufRead, writer: &mut impl Write) {
    let _: usize = read_value(read_line(reader));
    let s = read_line(reader);

    // noinspection SpellCheckingInspection
    const CHECKING_TARGETS: &str = "rsefaqtdwczxvg";
    let ans = match s.chars().last() {
        Some(last_char) if CHECKING_TARGETS.contains(last_char) => "1",
        _ => "0",
    };

    write!(writer, "{}", ans).expect("Failed to write");
}

// https://www.acmicpc.net/problem/25205
// noinspection SpellCheckingInspection
// 경로당펑크 2077
#[test]
fn test_solve25205() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "7
wnehgus"
                .to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "8
rlarudxo"
                .to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "10
anzufutaba"
                .to_string(),
            want: "1".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve25205(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
