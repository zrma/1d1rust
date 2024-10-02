use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve25314(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: i64 = read_line(reader).parse().unwrap();

    for _ in 0..(n / 4) {
        write!(writer, "long ").unwrap();
    }
    write!(writer, "int").unwrap();
}

// https://www.acmicpc.net/problem/25314
// 코딩은 체육과목 입니다
#[test]
fn test_solve25314() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "4".to_string(),
            want: "long int".to_string(),
        },
        TestData {
            s: "20".to_string(),
            want: "long long long long long int".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve25314(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
