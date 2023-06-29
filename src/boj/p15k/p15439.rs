use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve15439(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_line(reader).parse::<u64>().unwrap();
    write!(writer, "{}", n * (n - 1)).unwrap();
}

// https://www.acmicpc.net/problem/15439
// 베라의 패션
#[test]
fn test_solve15439() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestData {
            s: "1".to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "2".to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "3".to_string(),
            want: "6".to_string(),
        },
        TestData {
            s: "5".to_string(),
            want: "20".to_string(),
        },
        TestData {
            s: "100".to_string(),
            want: "9900".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve15439(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
