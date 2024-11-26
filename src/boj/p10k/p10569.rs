use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve10569(reader: &mut impl BufRead, writer: &mut impl Write) {
    let t = read_value(read_line(reader));
    for _ in 0..t {
        let (v, e) = read_values_as!(read_line(reader), i32, i32);

        writeln!(writer, "{}", 2 - v + e).expect("writeln! should work");
    }
}

// https://www.acmicpc.net/problem/10569
// 다면체
// noinspection SpellCheckingInspection
#[test]
fn test_solve10569() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "2
8 12
4 6"
            .to_string(),
            want: "6
4
"
            .to_string(),
        },
        TestData {
            s: "1
4 6"
            .to_string(),
            want: "4
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve10569(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
