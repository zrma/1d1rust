use crate::{read_values_as, utils::io::read_line};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve5691(reader: &mut impl BufRead, writer: &mut impl Write) {
    loop {
        let (a, b): (i32, i32) = read_values_as!(read_line(reader), i32, i32);
        if a == 0 && b == 0 {
            break;
        }

        writeln!(writer, "{}", find_min_third_num(a, b)).expect("Failed to write");
    }
}

fn find_min_third_num(a: i32, b: i32) -> i32 {
    debug_assert!(
        a >= 0 && a <= 1_000_000_000,
        "a must be between 0 and 1,000,000,000"
    );
    debug_assert!(
        b >= 0 && b <= 1_000_000_000,
        "b must be between 0 and 1,000,000,000"
    );
    2 * a - b
}

// https://www.acmicpc.net/problem/5691
// 평균 중앙값 문제
#[test]
fn test_solve5691() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "1 2
6 10
1 1000000000
0 0"
            .to_string(),
            want: "0
2
-999999998"
                .to_string(),
        },
        TestData {
            s: "10 20
0 0"
            .to_string(),
            want: "0".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve5691(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got.trim(), data.want, "failed at {} with {}", i, data.s);
    }
}
