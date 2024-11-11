use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve29720(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (n, m, k) = read_values_as!(read_line(reader), i32, i32, i32);

    let min_problems = (n - m * k).max(0);
    let max_problems = (n - (m * (k - 1) + 1)).max(0);

    write!(writer, "{} {}", min_problems, max_problems).expect("write should work");
}

// https://www.acmicpc.net/problem/29720
// 그래서 님 푼 문제 수가?
#[test]
fn test_solve29720() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "1000 5 128".to_string(),
            want: "360 364".to_string(),
        },
        TestData {
            s: "10 9 2".to_string(),
            want: "0 0".to_string(),
        },
        TestData {
            s: "10 2000 1".to_string(),
            want: "0 9".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve29720(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got.trim(), data.want, "failed at {} with {}", i, data.s);
    }
}
