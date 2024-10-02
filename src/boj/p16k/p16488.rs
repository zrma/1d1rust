use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve16488(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (n, k) = read_values_as!(read_line(reader), i64, i64);

    let res = n * n * k;
    write!(writer, "{}", res).expect("Failed to write");
}

// https://www.acmicpc.net/problem/16488
// 피카츄가 낸 어려운 문제
#[test]
fn test_solve16488() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "4 7".to_string(),
            want: "112".to_string(),
        },
        TestData {
            s: "1 1".to_string(),
            want: "1".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve16488(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
