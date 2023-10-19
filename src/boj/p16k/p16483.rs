use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve16483(reader: &mut impl BufRead, writer: &mut impl Write) {
    let t = read_line(reader).parse::<f64>().unwrap();
    let res = (t / 2.0).powi(2).round() as i32;
    write!(writer, "{}", res).unwrap();
}

// https://www.acmicpc.net/problem/16483
// 접시 안의 원
#[test]
fn test_solve16483() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "17".to_string(),
            want: "72".to_string(),
        },
        TestData {
            s: "18".to_string(),
            want: "81".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve16483(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "Failed test case {}", i);
    }
}