use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve20492(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: i64 = read_value(read_line(reader));

    // 첫 번째 경우: 전체 상금의 22%를 세금으로 내는 경우
    let case1 = n * 78 / 100;

    // 두 번째 경우: 상금의 80%를 필요 경비로 인정받고, 나머지 20%에 대해서만 22%의 세금을 내는 경우
    // 정확도를 위해 한 번에 계산: 80% + (20% * 78%) = 95.6%
    let case2 = n * 9560 / 10000;

    writeln!(writer, "{} {}", case1, case2).unwrap();
}

// https://www.acmicpc.net/problem/20492
// 세금
#[test]
fn test_solve20492() {
    struct TestCase {
        s: String,
        want: String,
    }
    for (i, tc) in [
        TestCase {
            s: "10000000".to_string(),
            want: "7800000 9560000".to_string(),
        },
        TestCase {
            s: "1000".to_string(),
            want: "780 956".to_string(),
        },
        TestCase {
            s: "1000000".to_string(),
            want: "780000 956000".to_string(),
        },
        TestCase {
            s: "100".to_string(),
            want: "78 95".to_string(),
        },
        TestCase {
            s: "1234567".to_string(),
            want: "962962 1180246".to_string(),
        },
        TestCase {
            s: "10000".to_string(),
            want: "7800 9560".to_string(),
        },
        TestCase {
            s: "999999".to_string(),
            want: "779999 955999".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = tc.s.as_bytes();
        let mut writer = vec![];
        solve20492(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got.trim(), tc.want.trim(), "failed at {} with {}", i, tc.s);
    }
}
