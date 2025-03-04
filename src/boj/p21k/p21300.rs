use crate::utils::io::read_values;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve21300(reader: &mut impl BufRead, writer: &mut impl Write) {
    let bottles: Vec<i32> = read_values(reader);
    let total: i32 = bottles.iter().sum::<i32>() * 5; // 각 병당 5센트씩 환불
    writeln!(writer, "{}", total).unwrap();
}

// https://www.acmicpc.net/problem/21300
// Bottle Return
#[test]
fn test_solve21300() {
    struct TestCase {
        s: String,
        want: String,
    }
    for (i, tc) in [
        // 공식 예제
        TestCase {
            s: "0 0 0 23 3 100".to_string(),
            want: "630".to_string(),
        },
        // 추가 테스트 케이스
        TestCase {
            s: "0 0 0 0 0 0".to_string(),
            want: "0".to_string(),
        },
        TestCase {
            s: "1 1 1 1 1 1".to_string(),
            want: "30".to_string(),
        },
        TestCase {
            s: "2 3 4 5 6 7".to_string(),
            want: "135".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = tc.s.as_bytes();
        let mut writer = vec![];
        solve21300(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got.trim(), tc.want.trim(), "failed at {} with {}", i, tc.s);
    }
}
