use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve15894(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_line(reader).parse::<u64>().unwrap();

    let res = 4 * n;

    write!(writer, "{}", res).unwrap();
}

// https://www.acmicpc.net/problem/15894
// 수학은 체육과목 입니다
#[test]
fn test_solve15894() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestData {
            s: "1".to_string(),
            want: "4".to_string(),
        },
        TestData {
            s: "2".to_string(),
            want: "8".to_string(),
        },
        TestData {
            s: "3".to_string(),
            want: "12".to_string(),
        },
        TestData {
            s: "4".to_string(),
            want: "16".to_string(),
        },
        TestData {
            s: "5".to_string(),
            want: "20".to_string(),
        },
        TestData {
            s: "100".to_string(),
            want: "400".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve15894(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}