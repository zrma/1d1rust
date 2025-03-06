use crate::utils::functions::multiply_fft;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve22193(reader: &mut impl BufRead, writer: &mut impl Write) {
    // N과 M은 실제로 사용하지 않음
    let _lengths = read_line(reader);

    // 두 숫자를 문자열로 읽어옴
    let num1 = read_line(reader);
    let num2 = read_line(reader);

    // 두 큰 정수를 곱셈
    let result = multiply_fft(&num1, &num2);

    writeln!(writer, "{}", result).unwrap();
}

// https://www.acmicpc.net/problem/22193
// Multiply
#[test]
fn test_solve22193() {
    struct TestCase {
        s: String,
        want: String,
    }
    for (i, tc) in [
        // 공식 예제
        TestCase {
            s: "3 4
123
4321"
                .to_string(),
            want: "531483".to_string(),
        },
        // 추가 테스트 케이스
        TestCase {
            s: "5 5
12345
67890"
                .to_string(),
            want: "838102050".to_string(),
        },
        TestCase {
            s: "1 1
0
0"
            .to_string(),
            want: "0".to_string(),
        },
        TestCase {
            s: "1 1
1
1"
            .to_string(),
            want: "1".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = tc.s.as_bytes();
        let mut writer = vec![];
        solve22193(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got.trim(), tc.want.trim(), "failed at {} with {}", i, tc.s);
    }
}
