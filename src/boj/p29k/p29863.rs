use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve29863(reader: &mut impl BufRead, writer: &mut impl Write) {
    let sleep_time: i32 = read_value(read_line(reader));
    let wake_time: i32 = read_value(read_line(reader));

    let sleep_hours = if sleep_time >= 20 && sleep_time <= 23 {
        24 - sleep_time + wake_time
    } else {
        wake_time - sleep_time
    };

    writeln!(writer, "{}", sleep_hours).unwrap();
}

// https://www.acmicpc.net/problem/29863
// Arno's Sleep Schedule
#[test]
fn test_solve29863() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "23
7"
            .to_string(),
            want: "8".to_string(),
        },
        TestData {
            s: "22
5"
            .to_string(),
            want: "7".to_string(),
        },
        TestData {
            s: "0
9"
            .to_string(),
            want: "9".to_string(),
        },
        TestData {
            s: "3
10"
            .to_string(),
            want: "7".to_string(),
        },
        TestData {
            s: "20
8"
            .to_string(),
            want: "12".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve29863(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(
            got.trim(),
            data.want.trim(),
            "failed at {} with {}",
            i,
            data.s
        );
    }
}
