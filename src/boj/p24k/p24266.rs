use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve24266(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_line(reader).parse::<i64>().unwrap();

    writeln!(writer, "{}", n * n * n).unwrap();
    writeln!(writer, "3").unwrap();
}

// https://www.acmicpc.net/problem/24266
// 알고리즘 수업 - 알고리즘의 수행 시간 5
#[test]
fn test_solve24266() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestData {
            s: "7".to_string(),
            want: "343\n3\n".to_string(),
        },
        TestData {
            s: "1".to_string(),
            want: "1\n3\n".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve24266(reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}