use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve10162(reader: &mut impl BufRead, writer: &mut impl Write) {
    let t: i32 = read_value(read_line(reader));
    let (a, b, c) = (t / 300, (t % 300) / 60, (t % 60) / 10);

    if t % 10 != 0 {
        write!(writer, "-1").expect("Failed to write");
    } else {
        write!(writer, "{} {} {}", a, b, c).expect("Failed to write");
    }
}

// https://www.acmicpc.net/problem/10162
// 전자레인지
#[test]
fn test_solve10162() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "100".to_string(),
            want: "0 1 4".to_string(),
        },
        TestData {
            s: "189".to_string(),
            want: "-1".to_string(),
        },
        TestData {
            s: "1000".to_string(),
            want: "3 1 4".to_string(),
        },
        TestData {
            s: "1890".to_string(),
            want: "6 1 3".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve10162(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
