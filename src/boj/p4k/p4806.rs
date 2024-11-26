use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve4806(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut count = 0;
    let mut line = String::new();

    while reader.read_line(&mut line).unwrap_or(0) > 0 {
        count += 1;
        line.clear();
    }

    write!(writer, "{}", count).expect("write! should work");
}

// https://www.acmicpc.net/problem/4806
// 줄 세기
#[test]
fn test_solve4806() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "Hello
Baekjoon
Online Judge"
                .to_string(),
            want: "3".to_string(),
        },
        TestData {
            s: "one


and two
three"
                .to_string(),
            want: "5".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve4806(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(
            got.trim(),
            data.want.trim(),
            "failed at {} with {}",
            i,
            data.s
        );
    }
}
