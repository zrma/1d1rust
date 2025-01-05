use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve19946(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: u128 = read_value(read_line(reader));
    let extra_operations = n.trailing_zeros(); // 실수 이후 추가 연산 횟수
    let ans = 64 - extra_operations; // 실수가 처음 발생한 위치
    writeln!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/19946
// 2의 제곱수 계산하기
#[test]
fn test_solve19946() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "18446744073709551615".to_string(),
            want: "64".to_string(),
        },
        TestData {
            s: "18446744073709551614".to_string(),
            want: "63".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        {
            let mut reader = &mut data.s.as_bytes();
            let mut writer = vec![];
            solve19946(&mut reader, &mut writer);

            let got = String::from_utf8(writer).unwrap();
            assert_eq!(
                got.trim(),
                data.want.trim(),
                "failed at {} with {}",
                i,
                data.s
            );
        }
    }
}
