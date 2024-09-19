use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve15610(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: f64 = read_value(read_line(reader));
    write!(writer, "{}", (n).sqrt() * 4.0).unwrap();
}

// https://www.acmicpc.net/problem/15610
// Abbey Courtyard
#[test]
fn test_solve15610() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "64".to_string(),
            want: "32.0".to_string(),
        },
        TestData {
            s: "1".to_string(),
            want: "4.0".to_string(),
        },
        TestData {
            s: "1234".to_string(),
            want: "140.51334456".to_string(),
        },
        TestData {
            s: "100".to_string(),
            want: "40".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve15610(&mut reader, &mut writer);

        let got: f64 =
            read_value(String::from_utf8(writer).expect("Failed to convert writer to string"));
        let want: f64 = data.want.parse().unwrap();

        assert!((got - want).abs() < 1e-6, "case {}", i);
    }
}
