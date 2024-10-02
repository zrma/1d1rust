use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve17863(reader: &mut impl BufRead, writer: &mut impl Write) {
    let s = read_line(reader);

    let res = if s.starts_with("555") { "YES" } else { "NO" };
    write!(writer, "{}", res).expect("Failed to write");
}

// https://www.acmicpc.net/problem/17863
// FYI
#[test]
fn test_solve17863() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5551212".to_string(),
            want: "YES".to_string(),
        },
        TestData {
            s: "5519876".to_string(),
            want: "NO".to_string(),
        },
        TestData {
            s: "5055555".to_string(),
            want: "NO".to_string(),
        },
        TestData {
            s: "5550000".to_string(),
            want: "YES".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve17863(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
