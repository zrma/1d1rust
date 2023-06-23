use num::integer::lcm;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve13241(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut line = String::new();
    let (a, b) = {
        reader.read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        let a = iter.next().unwrap().parse::<i64>().unwrap();
        let b = iter.next().unwrap().parse::<i64>().unwrap();
        (a, b)
    };

    let res = lcm(a, b);
    write!(writer, "{}", res).unwrap();
}

// https://www.acmicpc.net/problem/13241
// 최소공배수
#[test]
fn test_solve13241() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestData {
            s: "1 1".to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "3 5".to_string(),
            want: "15".to_string(),
        },
        TestData {
            s: "1 123".to_string(),
            want: "123".to_string(),
        },
        TestData {
            s: "121 199".to_string(),
            want: "24079".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve13241(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
