use crate::read_values_as;
use crate::utils::functions::multiply_fft;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve13277(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (a, b): (String, String) = read_values_as!(read_line(reader), String, String);
    let result = multiply_fft(&a, &b);
    writeln!(writer, "{}", result).unwrap();
}

// https://www.acmicpc.net/problem/13277
// 큰 수 곱셈
#[test]
fn test_solve13277() {
    struct TestCase {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestCase {
            s: "1 2".to_string(),
            want: "2".to_string(),
        },
        TestCase {
            s: "3 4".to_string(),
            want: "12".to_string(),
        },
        TestCase {
            s: "893724358493284 238947328947329".to_string(),
            want: "213553048277135320552236238436".to_string(),
        },
        TestCase {
            s: "1000000000000000000000000 1000000000000000000000000".to_string(),
            want: "1000000000000000000000000000000000000000000000000".to_string(),
        },
        TestCase {
            s: "1000000000000000000000000 0".to_string(),
            want: "0".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve13277(&mut reader, &mut writer);

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
