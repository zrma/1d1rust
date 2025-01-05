use crate::utils::io::read_values;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve11023(reader: &mut impl BufRead, writer: &mut impl Write) {
    let sum: i32 = read_values(reader).iter().sum();
    writeln!(writer, "{}", sum).unwrap();
}

// https://www.acmicpc.net/problem/11023
// 더하기 3
#[test]
fn test_solve11023() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "1 2 3 4 5".to_string(),
            want: "15".to_string(),
        },
        TestData {
            s: "1 2 3 4 5 6 7 8 9 10".to_string(),
            want: "55".to_string(),
        },
        TestData {
            s: "5 4 5 4 2 3 1 2".to_string(),
            want: "26".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve11023(&mut reader, &mut writer);

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
