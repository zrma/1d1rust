use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve24262(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_line(reader).parse::<i64>().unwrap();

    writeln!(writer, "{}", n).unwrap();
    writeln!(writer, "1").unwrap();
}

// https://www.acmicpc.net/problem/24262
// 알고리즘 수업 - 알고리즘의 수행 시간 2
#[test]
fn test_solve24262() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestData {
            s: "7".to_string(),
            want: "7\n1\n".to_string(),
        },
        TestData {
            s: "1".to_string(),
            want: "1\n1\n".to_string(),
        },
        TestData {
            s: "100000".to_string(),
            want: "100000\n1\n".to_string(),
        },
        TestData {
            s: "500000".to_string(),
            want: "500000\n1\n".to_string(),
        },
        TestData {
            s: "123".to_string(),
            want: "123\n1\n".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve24262(reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
