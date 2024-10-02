use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve20540(reader: &mut impl BufRead, writer: &mut impl Write) {
    let s = read_line(reader);

    let mut ans = String::new();
    for c in s.chars() {
        match c {
            'E' => ans.push('I'),
            'I' => ans.push('E'),
            'S' => ans.push('N'),
            'N' => ans.push('S'),
            'T' => ans.push('F'),
            'F' => ans.push('T'),
            'J' => ans.push('P'),
            'P' => ans.push('J'),
            _ => {}
        }
    }

    write!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/20540
// 연길이의 이상형
#[test]
fn test_solve20540() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "ESTJ".to_string(),
            want: "INFP".to_string(),
        },
        TestData {
            s: "INFP".to_string(),
            want: "ESTJ".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve20540(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
