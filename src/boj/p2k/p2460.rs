use crate::boj::p2k::p2455::calc_max_passengers;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2460(reader: &mut impl BufRead, writer: &mut impl Write) {
    let max_passengers = calc_max_passengers(reader, 10);

    write!(writer, "{}", max_passengers).expect("Failed to write");
}

// https://www.acmicpc.net/problem/2455
// noinspection SpellCheckingInspection
// 지능형 기차
#[test]
fn test_solve2460() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "0 32
3 13
28 25
17 5
21 20
11 0
12 12
4 2
0 8
21 0"
                .to_string(),
            want: "42".to_string(),
        },
        TestData {
            s: "0 1
0 0
0 0
0 0
0 0
0 0
0 0
0 0
0 0
1 0"
            .to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "0 100
100 0
0 0
0 0
0 0
0 0
0 0
0 0
0 99
99 0"
                .to_string(),
            want: "100".to_string(),
        },
        TestData {
            s: "0 99
99 0
0 0
0 0
0 0
0 0
0 0
0 0
0 100
100 0"
                .to_string(),
            want: "100".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve2460(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
